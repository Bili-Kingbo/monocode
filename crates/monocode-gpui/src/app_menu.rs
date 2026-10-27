//! The native menu bar, after the Tauri app's `menu.rs`. Items dispatch GPUI
//! actions, so their shortcuts follow the live keymap and items the focused
//! view cannot handle are disabled.
use crate::{
    Quit,
    file_editor::{Find, Format, ReloadFile, Replace, Save, ToggleAutosave, ToggleWordWrap},
    input,
    shell::*,
};
use gpui::{App, KeyBinding, Menu, MenuItem, actions};
use monocode_core::i18n::t;

actions!(application, [Hide, HideOthers, ShowAll]);

/// Opens a Help menu link in the browser.
#[derive(Clone, Debug, PartialEq, gpui::Action)]
#[action(namespace = application, no_json)]
pub(crate) struct OpenUrl(pub &'static str);

const REPO: &str = "https://github.com/hardbeat920/monocode";

pub(crate) fn bind(cx: &mut App) {
    cx.on_action(|_: &Hide, cx| cx.hide());
    cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
    cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
    cx.on_action(|action: &OpenUrl, cx| cx.open_url(action.0));
    if cfg!(target_os = "macos") {
        cx.bind_keys([
            KeyBinding::new("cmd-h", Hide, None),
            KeyBinding::new("cmd-alt-h", HideOthers, None),
            KeyBinding::new("cmd-m", MinimizeWindow, None),
        ]);
    }
}

pub(crate) fn install(prefs: &crate::settings::Preferences, cx: &mut App) {
    cx.set_menus(vec![
        Menu::new("Monocode Native").items([
            MenuItem::action(t("About Monocode Native"), About),
            MenuItem::separator(),
            MenuItem::action(t("Settings…"), OpenSettings),
            MenuItem::action(t("Check for Updates…"), CheckForUpdates),
            MenuItem::separator(),
            MenuItem::action(t("Hide Monocode Native"), Hide),
            MenuItem::action(t("Hide Others"), HideOthers),
            MenuItem::action(t("Show All"), ShowAll),
            MenuItem::separator(),
            MenuItem::action(t("Quit Monocode Native"), Quit),
        ]),
        Menu::new(t("File")).items([
            MenuItem::action(t("Open Project…"), OpenProject),
            MenuItem::action(t("Search…"), FocusSearch),
            MenuItem::action(t("Go to File…"), GoToFile),
            MenuItem::action(t("Command Palette…"), OpenCommandPalette),
            MenuItem::action(t("Find in Files…"), SearchFiles),
            MenuItem::separator(),
            MenuItem::action(t("Save"), Save),
            MenuItem::action(t("Reload File"), ReloadFile),
            MenuItem::action(t("Autosave"), ToggleAutosave).checked(prefs.autosave),
            MenuItem::separator(),
            MenuItem::action(t("New Tab"), NewCodex),
            MenuItem::action(t("New Claude Tab"), NewClaude),
            MenuItem::action(t("New Terminal"), NewTerminal),
            MenuItem::action(t("Split Pane Right"), SplitRight),
            MenuItem::action(t("Split Pane Down"), SplitDown),
            MenuItem::action(t("Close Pane"), CloseTab),
            MenuItem::action(t("Close Other Tabs"), CloseOtherTabs),
            MenuItem::action(t("Close All Tabs"), CloseAllTabs),
            MenuItem::separator(),
            MenuItem::action(t("Previous Tab"), PreviousTab),
            MenuItem::action(t("Next Tab"), NextTab),
            MenuItem::action(t("Back"), VisitBack),
            MenuItem::action(t("Forward"), VisitForward),
        ]),
        Menu::new(t("Edit")).items([
            MenuItem::action(t("Undo"), input::Undo),
            MenuItem::action(t("Redo"), input::Redo),
            MenuItem::separator(),
            MenuItem::action(t("Cut"), input::Cut),
            MenuItem::action(t("Copy"), input::Copy),
            MenuItem::action(t("Paste"), input::Paste),
            MenuItem::action(t("Select All"), input::SelectAll),
            MenuItem::separator(),
            MenuItem::action(t("Find"), Find),
            MenuItem::action(t("Replace"), Replace),
            MenuItem::action(t("Format Document"), Format),
            MenuItem::action(t("Word Wrap"), ToggleWordWrap).checked(prefs.file_word_wrap),
            MenuItem::separator(),
            MenuItem::action(t("Fold"), input::FoldCode),
            MenuItem::action(t("Unfold"), input::UnfoldCode),
            MenuItem::action(t("Fold All"), input::FoldAll),
            MenuItem::action(t("Unfold All"), input::UnfoldAll),
        ]),
        Menu::new(t("View")).items([
            MenuItem::action(t("Toggle Sidebar"), ToggleSidebar),
            MenuItem::action(t("Toggle Session Sidebar"), ToggleRightPanel),
            MenuItem::action(t("Toggle Layout"), ToggleLayout),
            MenuItem::action(t("Toggle Workspace Style"), ToggleWorkspaceStyle),
            MenuItem::action(t("Inbox"), OpenInbox),
            MenuItem::action(t("Notes"), OpenNotes),
            MenuItem::action(t("Toggle Terminal"), ToggleTerminal),
            MenuItem::action(t("Switch Model…"), SwitchModel),
            MenuItem::separator(),
            MenuItem::action(t("Previous Session"), PreviousSession),
            MenuItem::action(t("Next Session"), NextSession),
            MenuItem::action(t("Previous Project"), PreviousProject),
            MenuItem::action(t("Next Project"), NextProject),
            MenuItem::separator(),
            MenuItem::action(t("Focus Left"), FocusPaneLeft),
            MenuItem::action(t("Focus Right"), FocusPaneRight),
            MenuItem::action(t("Focus Up"), FocusPaneUp),
            MenuItem::action(t("Focus Down"), FocusPaneDown),
            MenuItem::separator(),
            MenuItem::action(t("Zoom In"), ZoomIn),
            MenuItem::action(t("Zoom Out"), ZoomOut),
            MenuItem::action(t("Reset Zoom"), ResetZoom),
            MenuItem::action(t("Reload"), Reload),
            MenuItem::separator(),
            MenuItem::action(t("Sidebar Appearance…"), SidebarAppearance),
        ]),
        // AppKit adds its window list here once the menu is named "Window",
        // so it is retitled only after installing.
        Menu::new("Window").items([
            MenuItem::action(t("Minimize"), MinimizeWindow),
            MenuItem::action(t("Zoom"), ZoomWindow),
        ]),
        Menu::new(t("Help")).items([
            MenuItem::action(t("MonoCode Website"), OpenUrl("https://usemono.dev")),
            MenuItem::action(t("View on GitHub"), OpenUrl(REPO)),
            MenuItem::separator(),
            MenuItem::action(
                t("Report a Bug…"),
                OpenUrl(concat!(
                    "https://github.com/hardbeat920/monocode",
                    "/issues/new?template=bug_report.yml"
                )),
            ),
            MenuItem::action(
                t("Request a Feature…"),
                OpenUrl(concat!(
                    "https://github.com/hardbeat920/monocode",
                    "/issues/new?template=feature_request.yml"
                )),
            ),
        ]),
    ]);
    #[cfg(target_os = "macos")]
    retitle_window_menu();
}

/// Shows the Window menu in the interface language once AppKit has found it
/// by its English name.
#[cfg(target_os = "macos")]
fn retitle_window_menu() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;
    use objc2_foundation::NSString;
    let Some(main) = MainThreadMarker::new() else {
        return;
    };
    let title = t("Window");
    let Some(item) = NSApplication::sharedApplication(main)
        .mainMenu()
        .and_then(|menu| menu.itemWithTitle(&NSString::from_str("Window")))
    else {
        return;
    };
    let title = NSString::from_str(title);
    item.setTitle(&title);
    if let Some(menu) = item.submenu() {
        menu.setTitle(&title);
    }
}
