use crate::{
    settings::{SettingsEvent, SettingsView, widgets::secondary_button},
    theme::*,
};
use gpui::{AnyElement, Context, SharedString, div, prelude::*, px};
use monocode_core::i18n::t;
impl SettingsView {
    pub(super) fn inbox_page(&mut self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut page = vec![self.project_notification_settings(cx)];
        for (id, title, description, label, hint) in [
            (
                "github",
                "GitHub",
                t("Pull requests, reviews, and issues, read through the GitHub CLI."),
                t("Connection"),
                t(
                    "Install GitHub CLI from cli.github.com, run gh auth login in a terminal, then check again.",
                ),
            ),
            (
                "gitlab",
                "GitLab",
                t("Merge requests from GitLab.com or a self-managed instance."),
                t("Connection"),
                t(
                    "Connect GitLab.com or a self-managed GitLab instance. Use a personal access token with API access; the token is stored locally and Disconnect deletes it.",
                ),
            ),
            (
                "azuredevops",
                "ADO",
                t("Pull requests and Boards work items from your ADO organization."),
                t("Connection"),
                t(
                    "Connect your ADO organization with a personal access token (Boards + Repos read & write for comments). The token is stored locally and Disconnect deletes it.",
                ),
            ),
            (
                "jira",
                "Jira",
                t("Jira Cloud issues from the projects you pick."),
                t("Connection"),
                t("Connect your Jira Cloud site with your email and an API token."),
            ),
            (
                "linear",
                "Linear",
                t("Issues assigned to you, from the teams you pick."),
                t("API key"),
                t(
                    "Create a personal API key in Linear → Settings → Security & Access. Disconnect deletes it.",
                ),
            ),
        ] {
            let (connected, connection_label) = self
                .ctx
                .inbox_connections
                .get(id)
                .cloned()
                .unwrap_or_else(|| (false, t("Checking connection…").into()));
            let status = div()
                .text_size(px(12.))
                .text_color(content(if connected { 0.7 } else { 0.5 }))
                .child(connection_label)
                .into_any_element();
            let controls = if id == "github" {
                vec![
                    status,
                    secondary_button(
                        "github-installation-guide",
                        t("Installation guide"),
                        false,
                        false,
                    )
                    .child(t("Installation guide"))
                    .on_click(|_, _, cx| cx.open_url("https://cli.github.com/"))
                    .into_any_element(),
                    secondary_button("github-check-again", t("Check again"), false, false)
                        .child(t("Check again"))
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(SettingsEvent::ManageInboxProvider("github".into()))
                        }))
                        .into_any_element(),
                ]
            } else {
                vec![
                    status,
                    secondary_button(
                        SharedString::from(format!("connect-{id}")),
                        if connected { t("Manage") } else { t("Connect") },
                        false,
                        false,
                    )
                    .child(if connected { t("Manage") } else { t("Connect") })
                    .on_click(cx.listener(move |_, _, _, cx| {
                        cx.emit(SettingsEvent::ManageInboxProvider(id.into()))
                    }))
                    .into_any_element(),
                ]
            };
            let row = self.row(None, false, label, Some(hint.into()), controls);
            page.push(
                self.group(
                    Some(id),
                    false,
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(icon(
                            match id {
                                "linear" => "inbox-linear",
                                "jira" => "inbox-jira",
                                "gitlab" => "inbox-gitlab",
                                "azuredevops" => "inbox-azuredevops",
                                _ => "inbox-github",
                            },
                            16.,
                        ))
                        .child(title),
                    Some(description.into()),
                    None,
                    vec![row],
                ),
            );
        }
        page
    }
}
