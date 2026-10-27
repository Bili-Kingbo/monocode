//! ChatPage.
use crate::settings::SettingsView;
use gpui::{AnyElement, Context};
use monocode_core::i18n::t;

impl SettingsView {
    pub(super) fn chat_page(&mut self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let p = self.prefs.clone();
        let transcript = vec![
            self.row(
                Some("transcript-layout"),
                true,
                t("Transcript layout"),
                Some(t("Show prompts at full width or as right-aligned chat bubbles.").into()),
                vec![self.segmented(
                    "transcript-layout",
                    t("Transcript layout"),
                    &p.transcript_layout,
                    &[("full", t("Full width")), ("chat", t("Chat"))],
                    cx,
                    |this, v, cx| this.update(cx, |p| p.transcript_layout = v.into()),
                )],
            ),
            self.row(
                Some("anchor-prompts"),
                false,
                t("Anchor prompts to top"),
                Some(t("Place new prompts at the top, with replies below.").into()),
                vec![self.toggle(
                    "anchor-prompts",
                    t("Anchor prompts to top"),
                    p.anchor_prompts,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.anchor_prompts = on),
                )],
            ),
            self.row(
                Some("paced-reveal"),
                false,
                t("Smooth text reveal"),
                Some(t("Reveal incoming text gradually.").into()),
                vec![self.toggle(
                    "paced-reveal",
                    t("Smooth text reveal"),
                    p.paced_reveal,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.paced_reveal = on),
                )],
            ),
        ];
        let composer = vec![
            self.row(
                Some("follow-up"),
                true,
                t("Follow-up behavior"),
                Some(t("Queue follow-ups until the active turn finishes, or steer the active turn immediately.").into()),
                vec![self.segmented(
                    "follow-up",
                    t("Follow-up behavior"),
                    &p.follow_up,
                    &[("queue", t("Queue")), ("steer", t("Steer"))],
                    cx,
                    |this, v, cx| this.update(cx, |p| p.follow_up = v.into()),
                )],
            ),
            self.row(
                Some("model-controls"),
                false,
                t("Model controls"),
                Some(t("Show model options beside the picker instead of inside the model menu.").into()),
                vec![self.segmented(
                    "model-controls",
                    t("Model controls"),
                    &p.model_controls,
                    &[("menu", t("Menu")), ("beside", t("Beside"))],
                    cx,
                    |this, v, cx| this.update(cx, |p| p.model_controls = v.into()),
                )],
            ),
            self.row(
                Some("composer-autocorrect"),
                false,
                t("Autocorrect"),
                Some(t("Spell check and autocorrect prompts.").into()),
                vec![self.toggle(
                    "composer-autocorrect",
                    t("Autocorrect"),
                    p.autocorrect,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.autocorrect = on),
                )],
            ),
        ];
        let editor = vec![self.row(
            Some("format-on-save"),
            true,
            t("Format on save"),
            Some(t("Format supported files with Prettier before saving.").into()),
            vec![self.toggle(
                "format-on-save",
                t("Format on save"),
                p.format_on_save,
                false,
                cx,
                |this, on, cx| this.update(cx, |p| p.format_on_save = on),
            )],
        )];
        let review = vec![self.row(
            Some("diff-view"),
            true,
            t("Diff view"),
            Some(t("Review changes in the editor or in a combined diff.").into()),
            vec![self.segmented(
                "diff-view",
                t("Diff view"),
                &p.diff_view,
                &[("editor", t("Editor")), ("unified", t("Unified"))],
                cx,
                |this, v, cx| this.update(cx, |p| p.diff_view = v.into()),
            )],
        )];
        let extras = vec![
            self.row(
                Some("composer-mascot"),
                true,
                t("Composer mascot"),
                Some(t("Show the project mascot while an agent works.").into()),
                vec![self.toggle(
                    "composer-mascot",
                    t("Composer mascot"),
                    p.composer_mascot,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.composer_mascot = on),
                )],
            ),
            self.row(
                Some("empty-session-games"),
                false,
                t("Empty session games"),
                Some(t("Show playable games in empty sessions.").into()),
                vec![self.toggle(
                    "empty-session-games",
                    t("Empty session games"),
                    p.empty_session_games,
                    false,
                    cx,
                    |this, on, cx| this.update(cx, |p| p.empty_session_games = on),
                )],
            ),
        ];
        vec![
            self.group(None, true, t("Transcript"), None, None, transcript),
            self.group(None, false, t("Composer"), None, None, composer),
            self.group(None, false, t("Editor"), None, None, editor),
            self.group(None, false, t("Code review"), None, None, review),
            self.group(None, false, t("Extras"), None, None, extras),
        ]
    }
}
