//! A folder that is not a Git checkout but holds several, such as a workspace
//! of projects: its checkouts with their branch and uncommitted counts, and
//! the one that Changes, History and the worktree list follow. Reads run
//! off-thread and never overlap.
use gpui::{Context, EventEmitter, Task};
use monocode_core::scm::{self, NestedRepo};
use std::{collections::HashMap, path::PathBuf, time::Duration};

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
            selected,
            in_flight: false,
            pending: false,
            poll: None,
        }
    }
    /// The current folder's checkouts, once read.
    pub(crate) fn repos(&self) -> &[NestedRepo] {
        self.scanned.get(&self.root).map_or(&[], Vec::as_slice)
    }
    /// The checkout Git views use for `folder`, if it holds any.
    pub(crate) fn target(&self, folder: &str) -> Option<&str> {
        let repos = self.scanned.get(folder)?;
        self.selected
            .get(folder)
            .and_then(|picked| repos.iter().find(|r| &r.path == picked))
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
        cx.notify();
        self.reload(cx);
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
        let path = PathBuf::from(&root);
        let work = cx
            .background_executor()
            .spawn(async move { scm::nested_repos(&path) });
        cx.spawn(async move |this, cx| {
            let repos = work.await;
            let _ = this.update(cx, |this, cx| {
                this.in_flight = false;
                // Until the user picks one, open on a checkout with changes,
                // and keep it while the others change.
                if !this.selected.contains_key(&root)
                    && let Some(first) = repos
                        .iter()
                        .find(|r| r.stats.files > 0)
                        .or_else(|| repos.first())
                {
                    this.selected.insert(root.clone(), first.path.clone());
                }
                if this.scanned.get(&root) != Some(&repos) {
                    this.scanned.insert(root, repos);
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
                    if !this.repos().is_empty() {
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
        nested.scanned.insert("/w".into(), vec![repo("/w/a", 0)]);
        assert_eq!(nested.selected(), Some("/w/a"));
        assert_eq!(nested.target("/elsewhere"), None);
    }
}
