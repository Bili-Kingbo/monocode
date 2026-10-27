use crate::{
    settings::{
        SettingsEvent, SettingsView,
        widgets::{note, secondary_button},
    },
    theme::*,
};
use gpui::{AnyElement, Context, Role, SharedString, div, prelude::*, px};
use monocode_core::{i18n::t, tr};
impl SettingsView {
    pub(super) fn archive_page(&mut self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut rows = vec![self.row(
            Some("show-archived"),
            true,
            t("Show archived in the sidebar"),
            Some(t("Keep archived conversations listed alongside the active ones.").into()),
            vec![self.toggle(
                "show-archived",
                t("Show archived in the sidebar"),
                self.prefs.show_archived,
                false,
                cx,
                |this, on, cx| this.update(cx, |p| p.show_archived = on),
            )],
        )];
        if self.ctx.cwd.is_empty() {
            rows.push(note(t("Open a project to see its archived conversations.")));
        } else if self.ctx.archived.is_empty() {
            rows.push(note(t("No archived conversations in this project.")));
        }
        for session in &self.ctx.archived {
            let id = session.id;
            let date = chrono::DateTime::from_timestamp_millis(
                session.updated_at.min(i64::MAX as u64) as i64,
            )
            .filter(|_| session.updated_at > 0)
            .map(|d| d.format(t("%b %-d")).to_string())
            .unwrap_or_default();
            rows.push(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .border_t_1()
                    .border_color(content(0.05))
                    .px_4()
                    .py(px(10.))
                    .child(provider_icon(&session.provider, 14.))
                    .child(
                        div()
                            .id(SharedString::from(format!("archived-session-{id}")))
                            .role(Role::Button)
                            .aria_label(session.title.clone())
                            .min_w_0()
                            .flex_1()
                            .truncate()
                            .text_size(px(13.))
                            .cursor_pointer()
                            .child(session.title.clone())
                            .on_click(cx.listener(move |_, _, _, cx| {
                                cx.emit(SettingsEvent::OpenSession(id))
                            })),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(content(0.35))
                            .child(date),
                    )
                    .child(
                        secondary_button(
                            SharedString::from(format!("unarchive-{id}")),
                            t("Unarchive"),
                            false,
                            false,
                        )
                        .child(t("Unarchive"))
                        .on_click(cx.listener(move |_, _, _, cx| {
                            cx.emit(SettingsEvent::Archive(id, false))
                        })),
                    )
                    .child(
                        secondary_button(
                            SharedString::from(format!("delete-{id}")),
                            t("Delete"),
                            true,
                            false,
                        )
                        .child(t("Delete"))
                        .on_click(
                            cx.listener(move |_, _, _, cx| cx.emit(SettingsEvent::Delete(id))),
                        ),
                    )
                    .into_any_element(),
            );
        }
        let title = if self.ctx.cwd.is_empty() {
            t("Archived conversations").into()
        } else {
            tr!("Archived in {project}", project = self.ctx.project_label)
        };
        let mut projects: Vec<AnyElement> = self
            .ctx
            .archived_projects
            .iter()
            .enumerate()
            .map(|(ix, (path, label))| {
                let (unarchive, delete) = (path.clone(), path.clone());
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .when(ix > 0, |d| d.border_t_1().border_color(content(0.05)))
                    .px_4()
                    .py(px(10.))
                    .child(
                        div()
                            .min_w_0()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(div().truncate().text_size(px(13.)).child(label.clone()))
                            .child(
                                div()
                                    .truncate()
                                    .text_size(px(11.))
                                    .text_color(content(0.35))
                                    .child(path.clone()),
                            ),
                    )
                    .child(
                        secondary_button(
                            SharedString::from(format!("unarchive-project-{ix}")),
                            t("Unarchive"),
                            false,
                            false,
                        )
                        .child(t("Unarchive"))
                        .on_click(cx.listener(move |_, _, _, cx| {
                            cx.emit(SettingsEvent::UnarchiveProject(unarchive.clone()))
                        })),
                    )
                    .child(
                        secondary_button(
                            SharedString::from(format!("delete-project-{ix}")),
                            t("Delete"),
                            true,
                            false,
                        )
                        .child(t("Delete"))
                        .on_click(cx.listener(move |_, _, _, cx| {
                            cx.emit(SettingsEvent::DeleteProject(delete.clone()))
                        })),
                    )
                    .into_any_element()
            })
            .collect();
        if projects.is_empty() {
            projects.push(note(t("No archived projects.")));
        }
        vec![
            self.group(
                None,
                true,
                t("Archived projects"),
                Some(t("Archive a project from the rail to keep its chats without listing it in the sidebar.").into()),
                None,
                projects,
            ),
            self.group(None, false, SharedString::from(title), None, None, rows),
        ]
    }
}
