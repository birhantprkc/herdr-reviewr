---
Status: Draft
Created: 2026-06-23
Last edited: 2026-06-27
---

# herdr-reviewr

herdr-reviewr is a terminal review sidebar that runs in a herdr pane, where you browse a coding agent's changes, comment on line ranges, and send those comments back to the agent.

## Overview

The product is one binary (`herdr-reviewr`, Rust + ratatui) in a right-hand herdr split pane, pointed at one git worktree. It never edits the worktree and sends nothing on its own; its only git write is a private `last-turn` baseline ref (`herdr-host.md`). It renders in your real terminal, so fonts and theming are whatever you already run.

A reviewer's loop:

```
open the pane → pick a changed file → read its diff → comment on a range
→ Send all your comments to the agent → add a line and hit enter
```

The product is a review cockpit of three tabs: a changes-and-diff reviewer (`Changes`), a whole-repo file browser (`All files`), and a read-only PR mirror (`PR`) that brings the pull request's state, checks, and reviewer comments into the sidebar so you never leave the terminal to see them.

## Scope

In scope for this design:

- The `Changes` view: a changed-files list for a scope, plus a syntax-highlighted diff viewer (`diff-view.md`).
- The `All files` tab: a whole-repo file tree with a read-and-comment content viewer, annotated with the active scope's changes (`file-list.md`, `diff-view.md`).
- The `PR` tab: a read-only mirror of the pull request — identity, state, checks, and comments — read from GitHub, with external links only (`forge-host.md`, `tui.md`).
- Three scopes — `uncommitted`, `branch`, and `last-turn` — defined in `review-model.md`.
- Comments anchored to `path:start-end`, held in memory for the review pass.
- Export of all comments to the agent (filling its input) or to the clipboard.
- Poll-based refresh and a manual refresh key.
- Keyboard and mouse input, defined in `tui.md`.

## Roadmap

Named so the architecture stays open to them. None is part of this design.

- Reviewed-file state — marking a file reviewed and greying it in the list.
- Hopping between the agent's changed files while browsing `All files`.
- A side-by-side split diff view, for wide panes.
- Search within the diff, and live theme switching.

## Invariants

- The sidebar never commits, stages, or mutates the worktree, the index, or any branch; its one git write is the private `last-turn` baseline ref under `refs/reviewr/`.
- The sidebar never writes to GitHub — it only reads a pull request through `gh`, and never posts, resolves, re-runs, or merges (`forge-host.md`).
- A comment, saved or being typed, is never lost to a refresh or the agent's edits; only you remove it.
- Comments leave only by an explicit export, to the agent pane or the clipboard.
- The `PR` tab never writes to GitHub or routes to the agent — it only reads a pull request through `gh` and opens links in the browser (`forge-host.md`).
- The crate forbids `unsafe`.

## Decisions

- Lightweight in-memory comments, sent to the agent — matches a few-comments-then-prompt loop; a durable, stateful comment store (Conductor-style) is more than this needs.
- `All files` is a content browser you can comment in, not a second diff — it renders whole-file content and overlays the active scope's change markers, reusing the diff viewer and the navigator rather than a separate stack. Rejected: a read-only browser with no commenting.
- One authored-comment set across `Changes` and `All files` — a comment made in either shares the in-memory list and exports together, so a review pass is one set, not one per tab. Rejected: per-tab comment lists.
- The `PR` tab is read-only — its value is reading the PR's state, checks, and comments inside herdr without leaving for the web UI; making it act (route to the agent, merge, resolve) would add a write surface and a state machine the mirror does not need. Rejected: an action-oriented PR tab that routes feedback to the agent.
- A separate `forge-host.md` for GitHub, distinct from `herdr-host.md` — the IDE we run in and the forge we read from are different external systems, and conflating them leaks GitHub into the host doc. Rejected: one host spec.

## Open decisions

- None.

## Related specs

- `./review-model.md`
- `./diff-view.md`
- `./file-list.md`
- `./tui.md`
- `./herdr-host.md`
- `./forge-host.md`
