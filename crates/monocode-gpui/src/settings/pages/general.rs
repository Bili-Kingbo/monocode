//! GeneralPage and UpdateRow.
use crate::notifications::updater::{self, Action};
use crate::settings::{
    SettingsView,
    widgets::{primary_button, secondary_button},
};
use crate::theme::*;
use gpui::{AnyElement, Context, FontWeight, div, prelude::*, px};
use monocode_core::{i18n::t, tr};

impl SettingsView {
    pub(super) fn general_page(&mut self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let p = self.prefs.clone();
        let language = vec![self.row(
            Some("language"),
            true,
            t("Language"),
            Some(t("System follows your Mac's preferred language.").into()),
            vec![self.segmented(
                "language",
                t("Language"),
                &p.language,
                // Each language keeps its own name.
                &[
                    ("system", t("System")),
                    ("en", "English"),
                    ("zh-CN", "简体中文"),
                ],
                cx,
                |this, v, cx| this.update(cx, |p| p.language = v.into()),
            )],
        )];
        let alerts = vec![
            self.row(
                Some("sounds"),
                true,
                t("Sounds"),
                Some(t("Play sounds for activity, completed turns, and updates.").into()),
                vec![self.toggle(
                    "sounds",
                    t("Sounds"),
                    p.sounds,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.sounds = on),
                )],
            ),
            self.row(
                Some("notifications"),
                false,
                t("Notifications"),
                Some(t("Alert when reminders are due or agents finish or need input.").into()),
                {
                    let mut controls = self.notification_permission_controls(cx);
                    controls.push(self.toggle(
                        "notifications",
                        t("Notifications"),
                        p.notifications,
                        false,
                        cx,
                        |this, on, cx| {
                            this.update(cx, |p| p.notifications = on);
                            if on {
                                this.probe_notification_permission(true, cx);
                            }
                        },
                    ));
                    controls
                },
            ),
        ];
        let mut workspace = vec![
            self.row(
                Some("file-tabs"),
                true,
                t("File tabs"),
                Some(t("Choose where files open.").into()),
                vec![self.segmented(
                    "file-tabs",
                    t("File tabs"),
                    &p.file_tabs,
                    &[("pane", t("Beside chat")), ("workspace", t("Top bar"))],
                    cx,
                    |this, v, cx| this.update(cx, |p| p.file_tabs = v.into()),
                )],
            ),
            self.row(
                Some("file-word-wrap"),
                false,
                t("Word wrap"),
                Some(t("Wrap long lines to fit the editor.").into()),
                vec![self.toggle(
                    "file-word-wrap",
                    t("Word wrap"),
                    p.file_word_wrap,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.file_word_wrap = on),
                )],
            ),
            self.row(
                Some("tab-animations"),
                false,
                t("Tab animations"),
                Some(t("Animate tabs as they open and close.").into()),
                vec![self.toggle(
                    "tab-animations",
                    t("Tab animations"),
                    p.tab_animations,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.tab_animations = on),
                )],
            ),
            self.row(
                Some("notes"),
                false,
                t("Notes"),
                Some(t("Show Notes in project navigation. Mention saved notes with @note.").into()),
                vec![
                    self.toggle("notes", t("Notes"), p.notes, false, cx, |this, on, cx| {
                        this.update(cx, |p| p.notes = on)
                    }),
                ],
            ),
        ];
        if cfg!(target_os = "macos") {
            workspace.push(self.row(
                Some("quick-composer"),
                false,
                t("Quick composer"),
                Some(tr!(
                    "Press {shortcut} from any app. Return starts a session in the background; ⌘Return opens it. Change the shortcut in Keybindings.",
                    shortcut = super::keybindings::quick_composer_label(&p)
                ).into()),
                {let mut controls=Vec::new();if let Some(error)=&self.ctx.quick_error{controls.push(super::super::widgets::note(error.clone()));}controls.push(self.toggle(
                    "quick-composer",
                    t("Quick composer"),
                    p.quick_composer,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.quick_composer = on),
                ));controls},
            ));
        }
        workspace.push(self.row(
            Some("working-agents"),
            false,
            t("Working agents"),
            Some(t("Show active chats across projects. Finished chats stay until opened.").into()),
            vec![self.toggle(
                "working-agents",
                t("Working agents"),
                p.working_agents,
                false,
                cx,
                |this, on, cx| this.update(cx, |p| p.working_agents = on),
            )],
        ));
        workspace.push(
            self.row(
                Some("action-opens-terminal"),
                false,
                t("Show terminal for actions"),
                Some(
                    t("Open the terminal when an action runs. Off, actions run in the background.")
                        .into(),
                ),
                vec![self.toggle(
                    "action-opens-terminal",
                    t("Show terminal for actions"),
                    p.action_opens_terminal,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.action_opens_terminal = on),
                )],
            ),
        );
        workspace.push(self.row(
            Some("terminal-in-panel"),
            false,
            t("Terminal in side panel"),
            Some(t("In the Focused layout, open the terminal as a tab in the right panel instead of below the chat.").into()),
            vec![self.toggle(
                "terminal-in-panel",
                t("Terminal in side panel"),
                p.terminal_in_panel,
                false,
                cx,
                |this, on, cx| this.update(cx, |p| p.terminal_in_panel = on),
            )],
        ));
        if cfg!(target_os = "windows") {
            workspace.push(self.row(
                Some("close-to-tray"),
                false,
                t("Close to tray"),
                Some(t("Closing a window hides it to the system tray instead of quitting, so running agents keep going. Reopen from the tray icon, and quit for real from its menu. Turn this off to have close end the window.").into()),
                vec![self.toggle(
                    "close-to-tray",
                    t("Close to tray"),
                    p.close_to_tray,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.close_to_tray = on),
                )],
            ));
        }
        let version = crate::notifications::updater::version();
        let about = vec![
            self.row(
                Some("update"),
                true,
                div()
                    .flex()
                    .items_baseline()
                    .gap_2()
                    .child(t("Version"))
                    .child(
                        div()
                            .font_family(crate::fonts::mono())
                            .text_size(px(12.))
                            .font_weight(FontWeight::NORMAL)
                            .text_color(content(0.45))
                            .child(version),
                    ),
                Some(
                    cx.try_global::<updater::Shared>()
                        .and_then(|shared| updater::status(&shared.0, cx))
                        .unwrap_or_else(|| {
                            t("View release notes and check for app and agent updates.").into()
                        })
                        .into(),
                ),
                vec![
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            secondary_button("whats-new", t("What's new"), false, false)
                                .child(t("What's new"))
                                .on_click(cx.listener(|_, _, _, cx| {
                                    cx.emit(crate::settings::SettingsEvent::WhatsNew)
                                })),
                        )
                        .child(
                            secondary_button(
                                "check-harness-updates",
                                t("Check harness updates"),
                                false,
                                false,
                            )
                            .child(t("Check harness updates"))
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(crate::settings::SettingsEvent::CheckHarnessUpdates)
                            })),
                        )
                        .child(self.app_update_button(cx))
                        .into_any_element(),
                ],
            ),
        ];
        vec![
            self.group(None, true, t("Language"), None, None, language),
            self.group(None, false, t("Alerts"), None, None, alerts),
            self.group(None, false, t("Workspace"), None, None, workspace),
            self.group(None, false, t("About"), None, None, about),
        ]
    }
}
impl SettingsView {
    /// Check for updates, or once one is found, install it in place.
    fn app_update_button(&self, cx: &mut Context<Self>) -> AnyElement {
        let updater = cx
            .try_global::<updater::Shared>()
            .map(|shared| shared.0.clone());
        match updater.as_ref().map(|u| updater::action(u, cx)) {
            Some(Action::Install(version)) => {
                let label = tr!("Update to {version}", version = version);
                let mut ink = shell_fill();
                ink.a = 1.;
                primary_button("check-updates", label.clone(), false)
                    .child(icon("arrow-down-circle", 14.).text_color(ink))
                    .child(label)
                    .on_click(move |_, _, cx| {
                        if let Some(updater) = &updater {
                            updater::install(updater, cx)
                        }
                    })
                    .into_any_element()
            }
            Some(Action::Busy(label)) => {
                secondary_button("check-updates", label.clone(), false, true)
                    .child(icon("loader", 14.).text_color(content(0.7)))
                    .child(label)
                    .into_any_element()
            }
            Some(Action::Check) | None => {
                secondary_button("check-updates", t("Check for updates"), false, false)
                    .child(icon("refresh-cw", 14.).text_color(content(0.7)))
                    .child(t("Check for updates"))
                    .on_click(cx.listener(|_, _, _, cx| {
                        cx.emit(crate::settings::SettingsEvent::CheckAppUpdates)
                    }))
                    .into_any_element()
            }
        }
    }
}
