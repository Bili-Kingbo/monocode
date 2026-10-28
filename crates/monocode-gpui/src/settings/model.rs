//! Port of src/features/settings/model/settings.ts: the pages, their rail
//! groups and the index Settings search jumps through.
use monocode_core::{i18n::tl, n};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Section {
    #[default]
    General,
    Connections,
    Appearance,
    Keybindings,
    Chat,
    Providers,
    Mcp,
    Skills,
    Monos,
    Inbox,
    Archive,
    Worktrees,
}

pub(crate) const GROUPS: [(&str, &[Section]); 3] = [
    (
        n!("App"),
        &[
            Section::General,
            Section::Connections,
            Section::Appearance,
            Section::Keybindings,
        ],
    ),
    (
        n!("Agents"),
        &[
            Section::Chat,
            Section::Providers,
            Section::Mcp,
            Section::Skills,
            Section::Monos,
        ],
    ),
    (
        n!("Workspace"),
        &[Section::Inbox, Section::Archive, Section::Worktrees],
    ),
];

impl Section {
    pub(crate) const ALL: [Section; 12] = [
        Section::General,
        Section::Connections,
        Section::Appearance,
        Section::Keybindings,
        Section::Chat,
        Section::Providers,
        Section::Mcp,
        Section::Skills,
        Section::Monos,
        Section::Inbox,
        Section::Archive,
        Section::Worktrees,
    ];
    pub(crate) fn id(self) -> &'static str {
        match self {
            Section::General => "general",
            Section::Connections => "connections",
            Section::Appearance => "appearance",
            Section::Keybindings => "keybindings",
            Section::Chat => "chat",
            Section::Providers => "providers",
            Section::Mcp => "mcp",
            Section::Skills => "skills",
            Section::Monos => "monos",
            Section::Inbox => "inbox",
            Section::Archive => "archive",
            Section::Worktrees => "worktrees",
        }
    }
    pub(crate) fn label(self) -> &'static str {
        match self {
            Section::General => n!("General"),
            Section::Connections => n!("Connections"),
            Section::Appearance => n!("Appearance"),
            Section::Keybindings => n!("Keybindings"),
            Section::Chat => n!("Chat"),
            Section::Providers => n!("Providers"),
            Section::Mcp => "MCP",
            Section::Skills => n!("Skills"),
            Section::Monos => n!("Monos"),
            Section::Inbox => n!("Inbox"),
            Section::Archive => n!("Archive"),
            Section::Worktrees => n!("Worktrees"),
        }
    }
    pub(crate) fn description(self) -> &'static str {
        match self {
            Section::General => {
                n!("The build you are running, how MonoCode reaches you, and the panels it shows.")
            }
            Section::Connections => {
                n!("Connect your machines and run agents remotely through SSH.")
            }
            Section::Appearance => {
                n!("Theme, tint, translucency, workspace layout, and conversation backgrounds.")
            }
            Section::Keybindings => {
                n!("Every shortcut the workspace handles, from the app menu and the key handler.")
            }
            Section::Chat => {
                n!(
                    "How transcripts read, what the composer does with a follow-up, how files save, and how diffs open."
                )
            }
            Section::Providers => {
                n!(
                    "Provider accounts, agent CLIs MonoCode can drive, and the model new sessions start with."
                )
            }
            Section::Mcp => n!("Find MCP servers across providers and manage their connections."),
            Section::Skills => {
                n!("Discover and manage file skills from project, personal, and harness folders.")
            }
            Section::Monos => {
                n!("The resident agent beside your tabs, and which projects have one.")
            }
            Section::Inbox => {
                n!("Manage Inbox services and notification preferences for each project.")
            }
            Section::Archive => n!("Projects and conversations you have archived."),
            Section::Worktrees => n!("Manage additional worktrees for each project."),
        }
    }
    fn keywords(self) -> &'static str {
        match self {
            Section::General => "version update sounds notifications notes rail",
            Section::Connections => "ssh remote host machine server environment always on",
            Section::Appearance => {
                "theme dark light color accent glass blur zoom scale wallpaper rail sidebar"
            }
            Section::Keybindings => "shortcut hotkey keyboard binding",
            Section::Chat => {
                "transcript composer prompt message diff review layout format save editor"
            }
            Section::Providers => {
                "account sign in login model harness claude codex gemini cli default hooks"
            }
            Section::Mcp => {
                "tools servers connections oauth authenticate login claude codex cursor opencode"
            }
            Section::Skills => "skill instructions prompt",
            Section::Monos => "mono resident agent mascot claim project title bar",
            Section::Inbox => {
                "github gitlab linear jira atlassian azure devops connect token integration"
            }
            Section::Archive => "archived restore delete hidden",
            Section::Worktrees => "git branch worktree working copy project create delete",
        }
    }
    /// SettingsRail.tsx's SECTION_ICONS.
    pub(crate) fn icon(self) -> &'static str {
        match self {
            Section::General => "sliders",
            Section::Connections => "internet",
            Section::Appearance => "palette",
            Section::Keybindings => "keyboard",
            Section::Chat => "message-square",
            Section::Providers => "bot",
            Section::Mcp => "globe",
            Section::Skills => "sparkles",
            Section::Monos => "ghost",
            Section::Inbox => "inbox-hi",
            Section::Archive => "archive",
            Section::Worktrees => "folder-tree",
        }
    }
}

/// SETTINGS_INDEX: (id, section, label, keywords).
pub(crate) const INDEX: &[(&str, Section, &str, &str)] = &[
    (
        "remote-machines",
        Section::Connections,
        n!("Your machines"),
        "ssh remote connect host server environment",
    ),
    (
        "mobile-access",
        Section::Connections,
        n!("Your phones"),
        "phone iphone android mobile app pair qr code device revoke remove testflight",
    ),
    (
        "mobile-remote",
        Section::Connections,
        n!("Connect away from home"),
        "router port upnp nat remote address public hostname vpn wifi",
    ),
    (
        "mcp-servers",
        Section::Mcp,
        n!("MCP servers"),
        "claude tools connections oauth authenticate login add remove",
    ),
    (
        "monos-enabled",
        Section::Monos,
        n!("Show monos"),
        "mono agent rail hide",
    ),
    (
        "rail-monos-pinned",
        Section::Monos,
        n!("Pin monos to the icon rail"),
        "mono rail compact collapsed icons top divider project picker",
    ),
    (
        "mono-list",
        Section::Monos,
        n!("Your monos"),
        "mono reset soul name projects sessions sidebar visibility hidden show",
    ),
    (
        "project-worktrees",
        Section::Worktrees,
        n!("Project worktrees"),
        "git branch working copy create delete manage",
    ),
    (
        "language",
        Section::General,
        n!("Language"),
        "language locale translation chinese english 语言 中文 简体中文 英文",
    ),
    (
        "update",
        Section::General,
        n!("Version"),
        "update upgrade release what's new build changelog",
    ),
    (
        "sounds",
        Section::General,
        n!("Sounds"),
        "audio cue chime mute volume",
    ),
    (
        "notifications",
        Section::General,
        n!("Notifications"),
        "notify alert toast permission reminder background",
    ),
    (
        "notes",
        Section::General,
        n!("Notes"),
        "notebook markdown rail scratchpad",
    ),
    (
        "quick-composer",
        Section::General,
        n!("Quick composer"),
        "spotlight global shortcut hotkey floating prompt anywhere",
    ),
    (
        "working-agents",
        Section::General,
        n!("Working agents"),
        "live running sessions rail card",
    ),
    (
        "action-opens-terminal",
        Section::General,
        n!("Show terminal for actions"),
        "actions commands run script dock background",
    ),
    (
        "terminal-in-panel",
        Section::General,
        n!("Terminal in side panel"),
        "terminal dock right panel tab shell focused layout",
    ),
    (
        "file-tabs",
        Section::General,
        n!("File tabs"),
        "editor open top workspace normal session pane beside chat",
    ),
    (
        "file-word-wrap",
        Section::General,
        n!("Word wrap"),
        "files editor long lines horizontal scroll wrapping",
    ),
    (
        "tab-animations",
        Section::General,
        n!("Tab animations"),
        "motion open close resize transition",
    ),
    (
        "theme",
        Section::Appearance,
        n!("Theme"),
        "dark light system appearance mode",
    ),
    (
        "wayland-window-decorations",
        Section::Appearance,
        n!("Wayland window decorations"),
        "wayland linux compositor title bar titlebar border decorations minimize maximize close",
    ),
    (
        "workspace-theme",
        Section::Appearance,
        n!("Workspace theme"),
        "workspace theme color colour text accent syntax scheme preset harbor moss ember orchid",
    ),
    (
        "accent-color",
        Section::Appearance,
        n!("Accent color"),
        "highlight bubble send button tint",
    ),
    (
        "diff-colors",
        Section::Appearance,
        n!("Diff colors"),
        "colorblind color blind accessibility added removed red green blue orange high contrast changes",
    ),
    ("hue", Section::Appearance, n!("Hue"), "tint color chrome"),
    (
        "saturation",
        Section::Appearance,
        n!("Saturation"),
        "tint color neutral grey gray",
    ),
    (
        "dark-lightness",
        Section::Appearance,
        n!("Dark-mode lightness"),
        "black brightness contrast background",
    ),
    (
        "sidebar-opacity",
        Section::Appearance,
        n!("Sidebar opacity"),
        "glass translucent transparency vibrancy",
    ),
    (
        "main-pane-glass",
        Section::Appearance,
        n!("Main pane glass"),
        "translucent transparency body window",
    ),
    (
        "inset-layout",
        Section::Appearance,
        n!("Layout"),
        "classic focused new preview sidebar navigation tabs inset workspace card rounded floating linear frame",
    ),
    (
        "interface-scale",
        Section::Appearance,
        n!("Interface scale"),
        "zoom font size bigger smaller ui",
    ),
    (
        "collapsed-project-rail",
        Section::Appearance,
        n!("Collapsed project rail"),
        "sidebar compact icons hidden navigation layout",
    ),
    (
        "show-excluded-files",
        Section::Appearance,
        n!("Show excluded files"),
        "explorer gitignore ignored hidden files tree",
    ),
    (
        "chat-background",
        Section::Appearance,
        n!("Chat background"),
        "wallpaper image picture opacity backdrop",
    ),
    (
        "transcript-layout",
        Section::Chat,
        n!("Transcript layout"),
        "full width chat bubble message",
    ),
    (
        "anchor-prompts",
        Section::Chat,
        n!("Anchor prompts to top"),
        "scroll position sticky message",
    ),
    (
        "paced-reveal",
        Section::Chat,
        n!("Smooth text reveal"),
        "streaming pacing immediate latency performance animation",
    ),
    (
        "follow-up",
        Section::Chat,
        n!("Follow-up behavior"),
        "queue steer interrupt send while running",
    ),
    (
        "model-controls",
        Section::Chat,
        n!("Model controls"),
        "effort thinking reasoning fast service tier model picker composer",
    ),
    (
        "composer-autocorrect",
        Section::Chat,
        n!("Autocorrect"),
        "spelling spell check autocorrect typo macos composer",
    ),
    (
        "composer-mascot",
        Section::Chat,
        n!("Composer mascot"),
        "runner animation coin fun",
    ),
    (
        "format-on-save",
        Section::Chat,
        n!("Format on save"),
        "prettier quotes editor save format",
    ),
    (
        "diff-view",
        Section::Chat,
        n!("Diff view"),
        "unified editor review changes working tree",
    ),
    (
        "empty-session-games",
        Section::Chat,
        n!("Empty session games"),
        "pacman snake arcade grid fun",
    ),
    (
        "agent-clis",
        Section::Providers,
        n!("Agent CLIs"),
        "codex opencode cursor grok pi omp fx hermes antigravity devin binary path",
    ),
    (
        "provider-accounts",
        Section::Providers,
        n!("Provider accounts"),
        "account sign in login rename remove delete credentials profile usage limit quota exhausted",
    ),
    (
        "show-remaining-usage",
        Section::Providers,
        n!("Show remaining usage"),
        "usage limit meter bar left used quota percent",
    ),
    (
        "mask-emails",
        Section::Providers,
        n!("Mask account emails"),
        "email privacy blur hide screenshot account",
    ),
    (
        "claude-hooks",
        Section::Providers,
        n!("Claude Code hooks"),
        "pretooluse settings.json block command notification",
    ),
    (
        "idle-agents",
        Section::Providers,
        n!("Keep idle agents ready"),
        "memory ram cpu performance process warm park background",
    ),
    (
        "project-notifications",
        Section::Inbox,
        n!("Project notifications"),
        "mute resume sounds banners reminders categories",
    ),
    (
        "github",
        Section::Inbox,
        "GitHub",
        "gh cli connect pull request sign in",
    ),
    (
        "gitlab",
        Section::Inbox,
        "GitLab",
        "token self-managed merge request connect",
    ),
    (
        "azuredevops",
        Section::Inbox,
        "ADO",
        "azure devops boards repos pull request pat organization connect",
    ),
    (
        "jira",
        Section::Inbox,
        "Jira",
        "atlassian cloud site email api token issues projects connect",
    ),
    (
        "linear",
        Section::Inbox,
        "Linear",
        "api key issues teams connect",
    ),
    (
        "show-archived",
        Section::Archive,
        n!("Show archived in the sidebar"),
        "hidden conversations list",
    ),
];

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SearchResult {
    pub section: Section,
    pub setting: Option<&'static str>,
    pub label: &'static str,
}

/// Matches the English label and keywords, and the label as shown, so a
/// query in the interface language finds it too.
fn score(needle: &str, label: &str, keywords: &str) -> Option<f32> {
    let lower = label.to_lowercase();
    let shown = tl(label).to_lowercase();
    if lower.starts_with(needle) || shown.starts_with(needle) {
        Some(0.)
    } else if lower.contains(needle) || shown.contains(needle) {
        Some(1.)
    } else if keywords.to_lowercase().contains(needle) {
        Some(2.)
    } else {
        None
    }
}

/// searchSettings: individual settings first, then whole sections.
pub(crate) fn search(query: &str, compositor: &str) -> Vec<SearchResult> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut scored: Vec<(f32, SearchResult)> = Vec::new();
    for &(id, section, label, keywords) in INDEX {
        if id == "wayland-window-decorations" && compositor != "Wayland" {
            continue;
        }
        if let Some(s) = score(&needle, label, keywords) {
            scored.push((
                s,
                SearchResult {
                    section,
                    setting: Some(id),
                    label,
                },
            ));
        }
    }
    for section in Section::ALL {
        let keywords = format!(
            "{} {} {}",
            section.description(),
            tl(section.description()),
            section.keywords()
        );
        if let Some(s) = score(&needle, section.label(), &keywords) {
            scored.push((
                s + 0.5,
                SearchResult {
                    section,
                    setting: None,
                    label: section.label(),
                },
            ));
        }
    }
    scored.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| a.1.label.to_lowercase().cmp(&b.1.label.to_lowercase()))
    });
    scored.into_iter().take(8).map(|(_, r)| r).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wayland_decorations_are_searchable_only_on_wayland() {
        for query in ["Wayland", "window decorations", "compositor", "title bar"] {
            let results = search(query, "Wayland");
            let result = results
                .iter()
                .find(|r| r.setting == Some("wayland-window-decorations"))
                .unwrap();
            assert_eq!(result.section, Section::Appearance);
            for compositor in ["X11", "Headless", ""] {
                assert!(
                    search(query, compositor)
                        .iter()
                        .all(|r| r.setting != Some("wayland-window-decorations"))
                );
            }
        }
    }

    #[test]
    fn rows_win_their_own_name_and_shared_appearance_rows_are_searchable() {
        let results = search("theme", "Wayland");
        assert_eq!(results[0].setting, Some("theme"));
        assert_eq!(results[1].setting, Some("workspace-theme"));
        assert!(
            search("hue", "Wayland")
                .iter()
                .any(|r| r.setting == Some("hue"))
        );
        assert!(
            search("workspace theme", "Wayland")
                .iter()
                .any(|r| r.setting == Some("workspace-theme"))
        );
        let pages = search("archive", "Wayland");
        assert!(
            pages
                .iter()
                .any(|r| r.section == Section::Archive && r.setting.is_none())
        );
    }
}
