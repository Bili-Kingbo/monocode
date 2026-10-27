//! A folder that is not a Git checkout but holds several, such as a workspace
//! of projects: its checkouts with their branch and uncommitted counts, and
//! the one that Changes, History and the worktree list follow. A session's
//! own checkouts (those it edited, and the worktrees of its task branch) list
//! first, so a cross-repository task needs no picking. Reads run off-thread
//! and never overlap.
use gpui::{Context, EventEmitter, Task};
use monocode_core::scm::{self, NestedRepo};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::Duration,
};

/// While Git status is visible, the other checkouts' counts refresh this
/// often; the followed checkout has its own watcher.
const POLL: Duration = Duration::from_secs(30);

/// The user picked another checkout of `root`.
pub(crate) struct NestedRepoSelected {
    pub root: String,
    pub repo: String,
}

pub(crate) struct NestedRepos {
    root: String,
    /// Each read folder's checkouts; empty for a checkout or a plain folder.
    scanned: HashMap<String, Vec<NestedRepo>>,
    /// The active session's edited files, and the checkouts they lead to.
    session: Vec<String>,
    session_repos: Vec<NestedRepo>,
    /// Folders shown with all their checkouts instead of the session's.
    show_all: HashSet<String>,
    /// The followed checkout per folder. Saved picks come from the workspace.
    selected: HashMap<String, String>,
    in_flight: bool,
    pending: bool,
    poll: Option<Task<()>>,
}
impl EventEmitter<NestedRepoSelected> for NestedRepos {}

impl NestedRepos {
    pub(crate) fn new(selected: HashMap<String, String>) -> Self {
        Self {
            root: String::new(),
            scanned: HashMap::new(),
            session: Vec::new(),
            session_repos: Vec::new(),
            show_all: HashSet::new(),
            selected,
            in_flight: false,
            pending: false,
            poll: None,
        }
    }
    /// All of the current folder's checkouts, once read.
    pub(crate) fn all(&self) -> &[NestedRepo] {
        self.scanned.get(&self.root).map_or(&[], Vec::as_slice)
    }
    /// The active session's checkouts in the current folder.
    pub(crate) fn session_repos(&self) -> &[NestedRepo] {
        if self.all().is_empty() {
            &[]
        } else {
            &self.session_repos
        }
    }
    /// Whether the session's checkouts are listed rather than all of them.
    pub(crate) fn session_scope(&self) -> bool {
        !self.session_repos().is_empty() && !self.show_all.contains(&self.root)
    }
    /// The checkouts listed for the current folder.
    pub(crate) fn repos(&self) -> &[NestedRepo] {
        if self.session_scope() {
            self.session_repos()
        } else {
            self.all()
        }
    }
    /// The checkout Git views use for `folder`, if it holds any: the pick,
    /// else the first listed one with changes, else the first.
    pub(crate) fn target(&self, folder: &str) -> Option<&str> {
        let repos = if folder == self.root {
            self.repos()
        } else {
            self.scanned.get(folder).map_or(&[][..], Vec::as_slice)
        };
        self.selected
            .get(folder)
            .and_then(|picked| repos.iter().find(|r| &r.path == picked))
            .or_else(|| repos.iter().find(|r| r.stats.files > 0))
            .or_else(|| repos.first())
            .map(|r| r.path.as_str())
    }
    pub(crate) fn selected(&self) -> Option<&str> {
        self.target(&self.root)
    }
    pub(crate) fn set_root(&mut self, root: String, cx: &mut Context<Self>) {
        if root == self.root {
            return;
        }
        self.root = root;
        self.session_repos.clear();
        cx.notify();
        self.reload(cx);
    }
    /// The active session's edited files; its checkouts are read from them.
    pub(crate) fn set_session(&mut self, edited: &[String], cx: &mut Context<Self>) {
        if self.session == edited {
            return;
        }
        self.session = edited.to_vec();
        if self.session.is_empty() {
            self.session_repos.clear();
            cx.notify();
        } else if !self.all().is_empty() {
            self.reload(cx);
        }
    }
    pub(crate) fn set_show_all(&mut self, all: bool, cx: &mut Context<Self>) {
        let changed = if all {
            self.show_all.insert(self.root.clone())
        } else {
            self.show_all.remove(&self.root)
        };
        if changed {
            cx.notify();
        }
    }
    pub(crate) fn select(&mut self, repo: String, cx: &mut Context<Self>) {
        if self.selected().is_some_and(|current| current == repo) {
            return;
        }
        self.selected.insert(self.root.clone(), repo.clone());
        cx.emit(NestedRepoSelected {
            root: self.root.clone(),
            repo,
        });
        cx.notify();
    }
    pub(crate) fn reload(&mut self, cx: &mut Context<Self>) {
        if self.root.is_empty() || self.root == "~" {
            return;
        }
        if self.in_flight {
            self.pending = true;
            return;
        }
        self.in_flight = true;
        let root = self.root.clone();
        let session = self.session.clone();
        let edited = session.clone();
        let path = PathBuf::from(&root);
        let work = cx.background_executor().spawn(async move {
            let all = scm::nested_repos(&path);
            let mine = if all.is_empty() {
                Vec::new()
            } else {
                scm::session_repos(&path, &edited)
            };
            (all, mine)
        });
        cx.spawn(async move |this, cx| {
            let (all, mine) = work.await;
            let _ = this.update(cx, |this, cx| {
                this.in_flight = false;
                let current = this.root == root && this.session == session;
                let changed = this.scanned.get(&root) != Some(&all)
                    || (current && this.session_repos != mine);
                this.scanned.insert(root, all);
                if current {
                    this.session_repos = mine;
                }
                if changed {
                    cx.notify();
                }
                if std::mem::take(&mut this.pending) {
                    this.reload(cx);
                }
            });
        })
        .detach();
    }
    /// While Git status is visible, refresh a folder of checkouts now and
    /// then; a single checkout has nothing to read here.
    pub(crate) fn set_polling(&mut self, polling: bool, cx: &mut Context<Self>) {
        if polling == self.poll.is_some() {
            return;
        }
        if !polling {
            self.poll = None;
            return;
        }
        self.poll = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(POLL).await;
                let Ok(()) = this.update(cx, |this, cx| {
                    if !this.all().is_empty() {
                        this.reload(cx);
                    }
                }) else {
                    break;
                };
            }
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monocode_core::scm::DiffStats;
    fn repo(path: &str, files: i64) -> NestedRepo {
        NestedRepo {
            path: path.into(),
            name: path.rsplit('/').next().unwrap().into(),
            branch: Some("main".into()),
            stats: DiffStats {
                files,
                ..Default::default()
            },
        }
    }
    #[test]
    fn a_saved_pick_wins_while_its_checkout_remains() {
        let mut nested = NestedRepos::new(HashMap::from([("/w".into(), "/w/b".into())]));
        nested.root = "/w".into();
        nested
            .scanned
            .insert("/w".into(), vec![repo("/w/a", 0), repo("/w/b", 0)]);
        assert_eq!(nested.selected(), Some("/w/b"));
        nested
            .scanned
            .insert("/w".into(), vec![repo("/w/a", 0), repo("/w/c", 2)]);
        assert_eq!(nested.selected(), Some("/w/c"));
        assert_eq!(nested.target("/elsewhere"), None);
    }
    #[test]
    fn a_session_lists_its_own_checkouts_until_all_are_asked_for() {
        let mut nested = NestedRepos::new(HashMap::new());
        nested.root = "/w".into();
        nested
            .scanned
            .insert("/w".into(), vec![repo("/w/a", 0), repo("/w/b", 0)]);
        nested.session_repos = vec![repo("/w/worktrees/b-task", 1)];
        assert!(nested.session_scope());
        assert_eq!(nested.selected(), Some("/w/worktrees/b-task"));
        nested.show_all.insert("/w".into());
        assert_eq!(nested.repos().len(), 2);
        assert_eq!(nested.selected(), Some("/w/a"));
    }
}
