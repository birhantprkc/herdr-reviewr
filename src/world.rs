//! The world snapshot: the derived state one refresh produces, built from git alone.
//!
//! `build` reads nothing from `App`, so the same call runs synchronously today and behind
//! the worker later (specs/tui.md Refresh). Reconciling a snapshot into place state stays
//! in `App::reconcile_world`, the one home for the Continuity rules (specs/overview.md).

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::Result;

use crate::app::Tab;
use crate::file_list::{Annotation, Entry};
use crate::git;
use crate::model::Scope;

/// Everything the build reads. A landed snapshot reconciles only while the view still
/// matches the input that produced it (specs/tui.md).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldInput {
    pub repo: PathBuf,
    pub tab: Tab,
    pub scope: Scope,
    pub base: Option<String>,
    pub base_branches: Vec<String>,
    /// The `last-turn` baseline tree the changed set diffs against; `None` before a turn.
    pub turn_baseline: Option<String>,
    /// Expanded ignored directories whose children the `All files` tree loads.
    pub toggled_dirs: HashSet<String>,
}

/// The derived state one refresh produces: the scope changeset and the navigator entries.
#[derive(Debug)]
pub struct WorldSnapshot {
    pub changed: HashMap<String, Annotation>,
    pub entries: Vec<Entry>,
}

/// Build the snapshot for `input`. The changeset is computed regardless of tab so the
/// header count and comment staleness stay correct while `All files` lists the whole
/// worktree. In `last-turn` with no baseline yet, the changeset is empty until a turn
/// start is observed (specs/review-model.md).
pub fn build(input: &WorldInput) -> Result<WorldSnapshot> {
    let changed = match input.scope {
        Scope::LastTurn => match input.turn_baseline.as_deref() {
            Some(t) => git::changed_against_tree(&input.repo, t)?,
            None => Vec::new(),
        },
        _ => git::changed_files(
            &input.repo,
            input.scope,
            input.base.as_deref(),
            &input.base_branches,
        )?,
    };
    let changed_map: HashMap<String, Annotation> =
        changed.iter().map(|f| (f.path.clone(), Annotation::from(f))).collect();
    let entries = match input.tab {
        // The whole worktree (ignored included), with expanded ignored dirs loaded lazily.
        Tab::AllFiles => all_files_entries(input, &changed_map)?,
        // `Changes` (the `PR` tab never builds a snapshot).
        _ => changed.iter().map(Entry::from_changed).collect(),
    };
    Ok(WorldSnapshot { changed: changed_map, entries })
}

/// The `All files` entries: every worktree path (ignored dimmed), with the children of
/// expanded ignored directories loaded lazily (`specs/file-list.md`). Only directories the
/// user has expanded are walked, so the cost tracks what is on screen, not the whole tree.
pub(crate) fn all_files_entries(
    input: &WorldInput,
    changed: &HashMap<String, Annotation>,
) -> Result<Vec<Entry>> {
    let to_entry = |w: git::WorktreeEntry| Entry {
        annotation: changed.get(&w.path).cloned(),
        path: w.path,
        previous_path: None,
        ignored: w.ignored,
        is_dir: w.is_dir,
    };
    let mut entries: Vec<Entry> =
        git::all_files(&input.repo)?.into_iter().map(&to_entry).collect();
    let mut i = 0;
    while i < entries.len() {
        if entries[i].is_dir && input.toggled_dirs.contains(&entries[i].path) {
            let path = entries[i].path.clone();
            let children = git::list_ignored_dir(&input.repo, &path).into_iter().map(&to_entry);
            entries.extend(children);
        }
        i += 1;
    }
    Ok(entries)
}
