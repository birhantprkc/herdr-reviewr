# Collapsed folders with changes wear a dot

Status: Approved
Date: 2026-09-12

## Problem

On `All files`, folders start collapsed. A folder row is `▸ name/` and nothing else. A folder with changed files under it looks the same as one without.

The reviewer's question on that tab, with the tree collapsed, is which folders to open to find the change. Today the tree cannot answer it.

On `Changes`, every folder contains changes, so its presence is the answer. That tab has no gap.

Prior art: all-files trees (VS Code Explorer, Zed, nvim-tree, neo-tree) mark a folder whose descendants have changes. VS Code and Zed use a dim dot, deliberately not a status letter, since a folder mixes kinds. Changed-files-only trees (GitHub, GitLab, lazygit) mark folders with nothing.

## Proposal

On `All files`, a collapsed folder row with a changed file under it shows a `•` in the modified marker's color, right-aligned against the pane edge where a file row shows its stats. Expanded folder rows and `Changes` folder rows are unchanged.

Observable:

- On `All files` with the tree collapsed, `▸ src/                •` marks the folder holding changes. `▸ docs/` without a dot holds none.
- Expand `src/`. The row reads `▾ src/` and its children show their own markers and stats.
- A folder whose only change is zero-line (a pure rename, a binary) still wears the dot. The dot means "a changed file is under here", not a line count.
- A collapsed wholly-ignored directory placeholder shows no dot: its children are not listed until it is expanded. Expanding it loads them, and a kept ignored changed file then shows as itself.
- A single-child chain that already renders as a file row (`a/b/x.rs`) is unchanged: it carries the file's own marker.
- A git-ignored folder row keeps its dim name. The dot keeps its color, so a kept change under an ignored folder reads the same as anywhere else.
- The dot follows the active scope. Switching `Uncommitted` to `Branch` to `Last turn` repaints every dot on the tab, the same way file markers move. A near-empty tree on `Last turn` means no changes in that scope, not none at all.
- A folder whose only change is a file deleted on the branch or staged as deleted wears no dot. `All files` lists the worktree and index, and such a file has no row there, so there would be nothing to find. A file removed from the worktree but still in the index keeps its row and its marker, so its folder wears the dot.
- A folder name too wide for its row elides its head the way a file row does. On `All files` every folder row, dotted or not, keeps the dot's two columns free, so a name never shifts on toggle. A name within two columns of the edge elides where it fit before. On `Changes` nothing is reserved and only names that clip today change. In a pane narrower than the row's indent the row overflows the way a file row's stats do.

The tree build stores on every folder row whether a listed annotated entry lies under it, on both tabs and in both states. Rendering paints the dot only on `All files` and only while the row is collapsed. The build knows entries, the renderer knows the tab, and each gate lives where its fact is known.

### Decisions

- **A dot, not a letter or a count.** A folder mixes change kinds, so no letter is true of it. A count or line total is weight, and the question here is presence. The dot is the shape the all-files priors converged on. Tried in real panes and rejected: a grey dot (too faint against dimmed rows), a changed-file count, a larger `●`, and coloring the chevron. The small dot in the `M` marker's yellow won.
- **Collapsed only.** The priors keep the mark on expanded folders too. Here the expanded folder's children sit beneath it with their own markers, so the mark would repeat what the eye already sees. The accepted cost: a wide expanded folder whose marked file has scrolled off shows `▾ name/` with no hint until the reviewer scrolls. The reviewer just opened it, so they know it has changes. This is the one departure from the prior, chosen for a quieter tree.
- **`All files` only.** On `Changes` every folder has changes under it, so the dot would be on every row and say nothing.
- **From the tree's own entries.** The folder's dot must be true of the rows that appear when it opens. Summing the changeset by path prefix would mark a folder for a deleted file the tab has no row for, and the opened folder would show nothing. The placeholder gap above is the cost, and it closes on expand.

## Guarantees

1. **Dot means a listed change.** A collapsed `All files` folder wears the dot exactly when a listed entry with a change annotation lies under its path. Expanding it, and every folder under it, then reveals at least one marked file row. Falsified by a dot over a subtree with no marked file, or a marked listed file under a dotless collapsed folder. A collapsed ignored placeholder's children are not listed, so they do not count until it is expanded. Enforced in the row build, where the folder and its files come from the same entries.
2. **No dot elsewhere.** `Changes` folder rows and expanded folder rows carry no dot. Falsified by a dot on either. Enforced in the renderer's gate.
3. **Continuity holds.** The dot is derived state. A refresh may add or remove it. It moves no cursor, scroll, or fold. Enforced by storing it on the rebuilt row, never in place state.

## Alternatives

- **`+a −d` sums on collapsed folders.** Rejected: no tool puts line totals on a folder, and on `All files` the changeset has paths the tab has no row for, so the sum cannot be made consistent under toggle without new row kinds.
- **A changed-file count (`3 files`, JetBrains).** Rejected: weight, not presence, and a second grammar to fit in the row.
- **Dot on expanded folders too, and on `Changes`.** Rejected: repeats what the children already show, and on `Changes` it marks every row.
- **Propagate the child's status color to the folder name.** Rejected: the name column already encodes ignored-ness by color.

## Out of scope

- Folder line totals or file counts. If presence turns out not to be enough, that is its own change.
- The search screen and the `PR` tab navigator. Neither has folders.
- Listing deleted files on `All files`.

## Verification

- Unit (`file_list`): a folder over one annotated and one unannotated file is marked; a folder over unannotated files is not; a folder over one zero-line annotated file is marked. The mark is present on the row regardless of expansion.
- Render (`tests/render.rs`): on `All files`, a collapsed marked folder paints the right-aligned dot; the same folder expanded does not; on `Changes` a collapsed marked folder does not.
- Render: a scope switch on `All files` repaints the dots to the new scope's changeset.
- App: a refresh that adds a change under a collapsed folder leaves the cursor and folds where they were, and the dot appears.
- Render: a long folder name elides its head so the dot still fits, expanded or collapsed.
- Bench: `scripts/bench_tui.py` A/B before and after, since the build gains a fold over the tree and the folder row gains padding and elision. Expected within noise.
- Docs: the `All files` bullet in `README.md` gains the dot beside "Ignored paths show dimmed", and a CHANGELOG line.
