//! The interface language: English, which is the source text, or Simplified
//! Chinese. Labels stay English in the code and `t` looks each one up, so a
//! label without a translation still shows, in English. Both the native app
//! and the text core builds for it (tool rows, times) translate here.
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
};

mod zh_cn;

static CHINESE: AtomicBool = AtomicBool::new(false);

/// Applies the Language preference ("system", "en" or "zh-CN"). True when the
/// shown language changed, so open windows redraw in it.
pub fn set_language(language: &str) -> bool {
    let chinese = match language {
        "en" => false,
        "zh-CN" => true,
        _ => system_prefers_chinese(sys_locale::get_locales()),
    };
    CHINESE.swap(chinese, Ordering::Relaxed) != chinese
}

pub fn chinese() -> bool {
    CHINESE.load(Ordering::Relaxed)
}

/// The first preferred language the app has decides, as for bundled
/// localizations: Simplified Chinese, or English for anything else.
fn system_prefers_chinese(locales: impl Iterator<Item = String>) -> bool {
    for locale in locales {
        let locale = locale.to_ascii_lowercase().replace('_', "-");
        if locale == "zh" || locale.starts_with("zh-hans") || locale == "zh-cn" || locale == "zh-sg"
        {
            return true;
        }
        if locale.starts_with("en") || locale.starts_with("zh") {
            return false;
        }
    }
    false
}

fn table() -> &'static HashMap<&'static str, &'static str> {
    static TABLE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        zh_cn::ENTRIES
            .iter()
            .flat_map(|e| e.iter().copied())
            .collect()
    })
}

/// The shown text for an English label.
pub fn t(en: &'static str) -> &'static str {
    tl(en)
}

/// `t` for a label held in a variable, such as one from a table marked with
/// `n!`, or a fixed status kept as a `String`.
pub fn tl(en: &str) -> &str {
    if !chinese() {
        return en;
    }
    table().get(en).copied().unwrap_or(en)
}

/// A sentence with `{name}` slots, translated before the slots are filled so
/// each language can order them.
pub fn tf(en: &'static str, args: &[(&str, &dyn Display)]) -> String {
    let mut text = t(en).to_owned();
    for (name, value) in args {
        text = text.replace(&format!("{{{name}}}"), &value.to_string());
    }
    text
}

/// `tr!("Stage {count} files", count = n)`: `tf` with named slots.
#[macro_export]
macro_rules! tr {
    ($en:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::i18n::tf($en, &[$((stringify!($name), &$value as &dyn std::fmt::Display)),*])
    };
}

/// Marks a label in a `const` or `static` table for translation; it is shown
/// through `tl`.
#[macro_export]
macro_rules! n {
    ($en:literal) => {
        $en
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeSet, path::Path};

    fn slots(text: &str) -> BTreeSet<&str> {
        let mut slots = BTreeSet::new();
        let mut rest = text;
        while let Some(start) = rest.find('{') {
            let Some(len) = rest[start..].find('}') else {
                break;
            };
            slots.insert(&rest[start + 1..start + len]);
            rest = &rest[start + len + 1..];
        }
        slots
    }

    fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "i18n") {
                    continue;
                }
                sources(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs")
                && !path.ends_with("monocode-core/src/i18n.rs")
            {
                out.push((
                    path.display().to_string(),
                    std::fs::read_to_string(&path).unwrap(),
                ));
            }
        }
    }

    /// The English literals passed to `t(…)`, `tr!(…)` and `tf(…)`.
    fn labels(text: &str) -> Vec<String> {
        let mut labels = Vec::new();
        for call in ["t(\"", "tr!(\"", "tf(\"", "n!(\""] {
            let mut rest = text;
            while let Some(at) = rest.find(call) {
                let before = rest[..at].chars().next_back();
                rest = &rest[at + call.len()..];
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    continue;
                }
                let mut label = String::new();
                let mut chars = rest.chars();
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => match chars.next() {
                            Some('n') => label.push('\n'),
                            Some(other) => label.push(other),
                            None => break,
                        },
                        c => label.push(c),
                    }
                }
                labels.push(label);
            }
        }
        labels
    }

    #[test]
    fn every_label_in_the_source_has_a_chinese_translation() {
        let mut files = Vec::new();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        sources(&root.join("src"), &mut files);
        sources(&root.join("../monocode-gpui/src"), &mut files);
        let mut missing = BTreeSet::new();
        for (path, text) in &files {
            for label in labels(text) {
                if !table().contains_key(label.as_str()) {
                    missing.insert(format!("{path}: {label:?}"));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "untranslated labels:\n{}",
            missing.into_iter().collect::<Vec<_>>().join("\n")
        );
    }

    #[test]
    fn translations_keep_their_slots_and_agree() {
        let mut seen: HashMap<&str, &str> = HashMap::new();
        for (en, zh) in zh_cn::ENTRIES.iter().flat_map(|e| e.iter().copied()) {
            assert_eq!(slots(en), slots(zh), "{en:?} -> {zh:?}");
            if let Some(other) = seen.insert(en, zh) {
                assert_eq!(other, zh, "{en:?} is translated twice");
            }
        }
    }

    /// Chinese text uses full-width punctuation and no space between Chinese
    /// characters, as the rest of a Chinese interface does.
    #[test]
    fn translations_use_chinese_punctuation() {
        let cjk = |c: char| ('\u{4e00}'..='\u{9fff}').contains(&c);
        for (en, zh) in zh_cn::ENTRIES.iter().flat_map(|e| e.iter().copied()) {
            let chars: Vec<char> = zh.chars().collect();
            for (ix, c) in chars.iter().enumerate() {
                let prev = ix.checked_sub(1).map(|i| chars[i]);
                let next = chars.get(ix + 1).copied();
                if matches!(c, ',' | ';' | '!' | '?' | ':' | '(' | ')') {
                    assert!(
                        !prev.is_some_and(cjk) && !next.is_some_and(cjk),
                        "{en:?} -> {zh:?}: use full-width {c:?} beside Chinese"
                    );
                }
                if *c == ' ' {
                    assert!(
                        !(prev.is_some_and(cjk) && next.is_some_and(cjk)),
                        "{en:?} -> {zh:?}: no space between Chinese characters"
                    );
                }
            }
            // A space sets Latin text and numbers apart from Chinese.
            for pair in chars.windows(2) {
                let latin = |c: char| c.is_ascii_alphanumeric() || c == '}' || c == '{';
                assert!(
                    !(cjk(pair[0]) && latin(pair[1]) || latin(pair[0]) && cjk(pair[1])),
                    "{en:?} -> {zh:?}: put a space between Chinese and Latin text"
                );
            }
            if en.ends_with('…') {
                assert!(zh.ends_with('…'), "{en:?} -> {zh:?}: keep the ellipsis");
            }
        }
    }

    #[test]
    fn the_system_language_picks_simplified_chinese_only() {
        let pick = |list: &[&str]| system_prefers_chinese(list.iter().map(|s| s.to_string()));
        assert!(pick(&["zh-Hans-CN", "en-US"]));
        assert!(pick(&["zh_CN"]));
        assert!(!pick(&["en-US", "zh-Hans"]));
        assert!(!pick(&["zh-Hant-TW"]));
        assert!(pick(&["fr-FR", "zh-Hans"]));
        assert!(!pick(&[]));
    }

    #[test]
    fn labels_follow_the_language() {
        let was = chinese();
        CHINESE.store(true, Ordering::Relaxed);
        assert_eq!(t("Settings…"), "设置…");
        assert_eq!(t("Not a label anyone wrote"), "Not a label anyone wrote");
        assert_eq!(
            crate::tr!("Stage Changes in {path}", path = "src"),
            "暂存 src 中的改动"
        );
        CHINESE.store(false, Ordering::Relaxed);
        assert_eq!(
            crate::tr!("Stage Changes in {path}", path = "src"),
            "Stage Changes in src"
        );
        CHINESE.store(was, Ordering::Relaxed);
    }
}
