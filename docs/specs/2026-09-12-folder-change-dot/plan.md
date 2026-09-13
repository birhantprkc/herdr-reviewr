# Collapsed folders with changes wear a dot: Plan

Status: Approved

Delivers the sibling `spec.md`.

## Delivery

One repo, committed straight to `main`, no PR. One ticket, since the build change, the render gate, and their tests prove one behavior together and none lands safely alone. Merge after the user QAs the `just qa-install` build in real panes.

## Tickets

| Status | Ticket | Delivers | Depends on |
| --- | --- | --- | --- |
| Done | [01: folder change dot](#01-folder-change-dot) | A collapsed `All files` folder with a changed file under it shows a yellow right-aligned `•` | — |

## Triggers

- If `Row` gains a second folder-only fact, then split `RowKind::Dir` into a struct.

## Log

- 2026-09-13: initial plan.
- 2026-09-13: user chose no PR -> commit to `main` after QA -> Delivery section.
- 2026-09-13: QA in real panes -> grey dot too faint; count, large dot, colored chevron tried and rejected -> yellow `•`, spec Proposal and Decisions.
- 2026-09-13: user skipped the bench -> recorded as skipped in Evidence.

---

# 01: folder change dot

**Delivers:** On `All files`, a collapsed folder wears a `•` in the `M` marker's color at the right edge when a listed changed file lies under it. `Changes` rows are unchanged.

## Acceptance

- [x] Spec guarantee 1, the mark is on the row -> `file_list::tests::a_directory_row_marks_a_change_beneath_it`: a folder over one annotated and one unannotated file is marked. Over unannotated files only, not marked. Over one zero-line annotation (`additions: 0, deletions: 0`), marked. Marked the same whether expanded or collapsed. A nested marked folder marks every ancestor folder row.
- [x] Spec guarantee 1, the dot paints -> `render::a_collapsed_all_files_folder_with_a_change_wears_a_dot`: in a repo with `src/{app.rs,ui.rs}` and `docs/a.md` committed and `src/ui.rs` edited, on `All files` the `src/` row ends in `•` and the `docs/` row does not. Assert via `token_x` on the files pane, the way `an_expanded_directory_nests_its_children` does.
- [x] Spec guarantee 2, no dot elsewhere -> same render test, continued: `expand_dir` on `src/`, the row has no `•`. And `render::a_collapsed_changes_folder_wears_no_dot`: the same repo on `Changes` with `src/` collapsed via `collapse_dir` paints no `•`.
- [x] Spec Observable, scope follows -> `render::the_folder_dot_follows_the_scope`: edit committed on a branch, worktree clean. On `All files` under `Uncommitted`, no dot. `set_scope(Scope::Branch)` then `land_world`, the folder wears the dot.
- [x] Spec guarantee 3, continuity -> `app_flow::the_folder_dot_appears_under_a_poll_without_moving_the_cursor`: cursor parked on a collapsed unmarked folder on `All files`. Write a file under it, `land_world`. The cursor is on the same folder (by `dir_path`), the folder is still collapsed, and the row is marked. Copy the shape of `the_cursor_stays_on_a_folder_across_a_poll_and_toggle`.
- [x] Spec Observable, elision -> `render::a_long_folder_name_leaves_room_for_the_dot`: a folder whose name exceeds the pane width, collapsed with a change under it, renders `…/tail/` and the `•` in the last column. Expanded, the same elided name.
- [ ] Bench (skipped, see Evidence) -> `python3 scripts/bench_tui.py --binary target/release/herdr-reviewr --fixture` A/B against a `main` build in a second target dir, interleaved, on a quiet system. Medians within noise. Record both in Evidence. Replace `scripts/bench-results/baseline.json` only if the numbers moved.
- [x] Docs -> the `All files` bullet in `README.md` says a collapsed folder with changes shows a dot. `CHANGELOG.md` `Unreleased` gains an `Added` entry, two lines.
- [x] `just ci` green.

## Plan

1. `src/file_list.rs`: add `has_change: bool` to `RowKind::Dir` and to the build-tree `Dir` node. In `insert`, as an annotated entry's path is walked, set `has_change` on every directory node along it. `flatten` copies the compressed node's flag onto the row, in both states. Update the two other constructors, the test literals in `src/app.rs` and `src/selection.rs`.
   Alternative weighed: a per-row recursive walk of the subtree at flatten time. Simpler to read, but a file at depth d is visited d times per rebuild, and `rebuild_file_rows` runs on every fold keystroke. Marking on insert is O(depth) per annotated entry and no walk at all.
2. `src/ui.rs` `render_file_list`, the `RowKind::Dir` arm: elide the bare name through `elide_head` and append the slash, with a budget that on `All files` keeps `DIR_DOT_RESERVE` columns free on every folder row so the name never moves on toggle. `Changes` reserves nothing. When `app.tab == Tab::AllFiles && !expanded && has_change`, pad to the right edge and push `•` in `kind_color(p, ChangeKind::Modified.marker())`. Ignored rows dim the name; the dot keeps its color.
3. Tests as listed. `file_list` tests use the existing `entries`/`file` helpers plus an unannotated `Entry` literal. Render tests slice the files pane by column (`FILES_X0..=FILES_X1`) and check the dot's cell symbol and color against the `M` marker's.
4. README and CHANGELOG.
5. `just ci`.

## Evidence

- `just ci`: fmt-check, clippy, 828 tests, release build, all green.
- Bench: skipped at the user's call. The build's added work is one boolean per directory node per annotated entry, and the render's is per visible folder row.
- Review: seven finders (lines, removed, callers, spec, tests, model, efficiency), findings fixed: the reserve on `Changes`, the per-row subtree walk, the untested dot color and column, the rename, placeholder, deleted-file, scope-return, and row-shift cases. Declined: the dot overflowing a pane narrower than the row's indent, which matches how a file row's stats overflow. Out of scope: two pre-existing paths the diff does not touch (a scope switch while the `PR` tab shows, a failed ignored-directory load).
- QA: the user compared grey dot, yellow dot, changed-file count, large dot, and colored chevrons in real panes and chose the yellow dot.
