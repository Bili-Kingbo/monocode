//! Simplified Chinese, by area. Keys are the English labels exactly as the
//! code writes them; see `i18n.rs` for the checks every entry passes.
mod app;
mod conversation;
mod core_text;
mod features;
mod git_files;
mod monos;
mod settings;
mod settings_pages;
mod shell;

pub(super) const ENTRIES: &[&[(&str, &str)]] = &[
    app::ENTRIES,
    conversation::ENTRIES,
    core_text::ENTRIES,
    features::ENTRIES,
    git_files::ENTRIES,
    monos::ENTRIES,
    settings::ENTRIES,
    settings_pages::ENTRIES,
    shell::ENTRIES,
];
