//! Configuration: command-line flags.
//!
//! See `specs/tui.md` and `specs/herdr-host.md`. Flags override defaults; the positional
//! argument (if any) is the repo path, else the current directory.

use std::path::PathBuf;
use std::time::Duration;

/// Resolved runtime configuration.
#[derive(Clone, Debug)]
pub struct Config {
    pub repo: PathBuf,
    pub poll: Duration,
    pub base: Option<String>,
    pub theme: Option<String>,
    /// `Some(false)` when `--wrap off` is passed; `None` keeps the default (wrap on).
    pub wrap: Option<bool>,
}

impl Config {
    /// Parse `args` (the process arguments *after* argv\[0\]).
    ///
    /// Recognises `--poll <ms>` (min 200, default 2000), `--base <ref>`,
    /// `--theme <name>`, and `--wrap on|off`; the first non-flag token is the repo path.
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Self {
        let mut repo: Option<PathBuf> = None;
        let mut poll_ms: u64 = 2000;
        let mut base: Option<String> = None;
        let mut theme: Option<String> = None;
        let mut wrap: Option<bool> = None;
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--poll" => {
                    if let Some(v) = it.next() {
                        poll_ms = v.parse().unwrap_or(poll_ms);
                    }
                }
                "--base" => base = it.next(),
                "--theme" => theme = it.next(),
                "--wrap" => wrap = it.next().map(|v| v != "off"),
                other if !other.starts_with('-') => repo = Some(PathBuf::from(other)),
                _ => {}
            }
        }
        let repo =
            repo.or_else(|| std::env::current_dir().ok()).unwrap_or_else(|| PathBuf::from("."));
        Self { repo, poll: Duration::from_millis(poll_ms.max(200)), base, theme, wrap }
    }

    /// Parse from the real process arguments.
    pub fn from_env() -> Self {
        Self::parse(std::env::args().skip(1))
    }
}

/// The `theme` value from reviewr's config file (`$HERDR_PLUGIN_CONFIG_DIR/config.toml`),
/// read on each refresh. `None` when the dir is unset (standalone), the file is absent or
/// unparseable, or it has no `theme` key — the caller then falls back to the default
/// (`specs/theme.md`).
pub fn config_file_theme() -> Option<String> {
    config_theme_in(std::env::var_os("HERDR_PLUGIN_CONFIG_DIR")?)
}

/// The `theme` key from `<dir>/config.toml`, or `None` if the file is absent, unparseable,
/// or has no `theme` key. Split from the env lookup so it is testable.
fn config_theme_in(dir: impl AsRef<std::path::Path>) -> Option<String> {
    let text = std::fs::read_to_string(dir.as_ref().join("config.toml")).ok()?;
    let table: toml::Table = text.parse().ok()?;
    table.get("theme").and_then(toml::Value::as_str).map(str::to_owned)
}

/// The built-in base-branch candidates for the `branch` scope, used when `config.toml`
/// sets no `base_branches` (`specs/review-model.md`).
pub const DEFAULT_BASE_BRANCHES: [&str; 4] = ["origin/main", "origin/master", "main", "master"];

/// The ordered base-branch candidates, read on each refresh from reviewr's config file
/// (`$HERDR_PLUGIN_CONFIG_DIR/config.toml`). Falls back to [`DEFAULT_BASE_BRANCHES`] when the
/// dir is unset (standalone), the file is absent or unparseable, or it has no `base_branches`
/// array (`specs/review-model.md`).
pub fn base_branches() -> Vec<String> {
    std::env::var_os("HERDR_PLUGIN_CONFIG_DIR")
        .and_then(base_branches_in)
        .unwrap_or_else(|| DEFAULT_BASE_BRANCHES.iter().map(|s| (*s).to_owned()).collect())
}

/// The `base_branches` array from `<dir>/config.toml`, or `None` if the file is absent,
/// unparseable, or has no non-empty `base_branches` array of strings. Non-string entries are
/// dropped; an all-non-string or empty array falls back like an absent key. Split from the env
/// lookup so it is testable.
fn base_branches_in(dir: impl AsRef<std::path::Path>) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(dir.as_ref().join("config.toml")).ok()?;
    let table: toml::Table = text.parse().ok()?;
    let list: Vec<String> = table
        .get("base_branches")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect();
    (!list.is_empty()).then_some(list)
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::time::Duration;

    fn parse(args: &[&str]) -> Config {
        Config::parse(args.iter().map(|s| (*s).to_string()))
    }

    #[test]
    fn defaults_when_no_args() {
        let c = parse(&[]);
        assert_eq!(c.poll, Duration::from_secs(2));
        assert_eq!(c.base, None);
    }

    #[test]
    fn flags_and_positional_repo() {
        let c = parse(&["--poll", "500", "--base", "origin/dev", "/tmp/work"]);
        assert_eq!(c.poll, Duration::from_millis(500));
        assert_eq!(c.base.as_deref(), Some("origin/dev"));
        assert_eq!(c.repo.to_str(), Some("/tmp/work"));
    }

    #[test]
    fn poll_has_a_floor() {
        assert_eq!(parse(&["--poll", "10"]).poll, Duration::from_millis(200));
        assert_eq!(parse(&["--poll", "garbage"]).poll, Duration::from_secs(2));
    }

    #[test]
    fn reads_theme_from_config_toml() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("config.toml"), "theme = \"gruvbox\"\n").unwrap();
        assert_eq!(super::config_theme_in(dir.path()), Some("gruvbox".to_string()));
    }

    #[test]
    fn missing_file_or_absent_key_is_none() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(super::config_theme_in(dir.path()), None);
        std::fs::write(dir.path().join("config.toml"), "poll = 500\n").unwrap();
        assert_eq!(super::config_theme_in(dir.path()), None);
    }

    #[test]
    fn reads_base_branches_from_config_toml() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("config.toml"),
            "base_branches = [\"origin/dev\", \"main\"]\n",
        )
        .unwrap();
        assert_eq!(
            super::base_branches_in(dir.path()),
            Some(vec!["origin/dev".to_string(), "main".to_string()])
        );
    }

    #[test]
    fn base_branches_missing_or_malformed_falls_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        assert_eq!(super::base_branches_in(dir.path()), None, "no file");
        std::fs::write(&path, "theme = \"x\"\n").unwrap();
        assert_eq!(super::base_branches_in(dir.path()), None, "absent key");
        std::fs::write(&path, "base_branches = []\n").unwrap();
        assert_eq!(super::base_branches_in(dir.path()), None, "empty array");
        std::fs::write(&path, "base_branches = [1, 2]\n").unwrap();
        assert_eq!(super::base_branches_in(dir.path()), None, "non-string entries");
        std::fs::write(&path, "base_branches = :::\n").unwrap();
        assert_eq!(super::base_branches_in(dir.path()), None, "unparseable");
    }
}
