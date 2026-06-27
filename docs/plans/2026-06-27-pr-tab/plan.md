# Read-only PR tab — Delivery Plan

**Specs:** ../../../specs/ — the living reference this plan delivers (`forge-host.md`, `tui.md`, `overview.md`)

## Milestone Map

1. **Read-only PR tab** — single milestone; mirror the branch's open PR (identity, state, checks, comments) in a third tab, with select-to-read and open-in-browser.

One milestone: the slice is read-only (no migration, write path, or public contract — no commitment boundary), and the `gh` data shapes are already spiked (no open information boundary).

## Goal

A third tab, `PR`, that reads the branch's open pull request from GitHub via `gh` and presents its identity, state, checks, and comments read-only, per `forge-host.md` and `tui.md`.

## Definition of Done

- `3` opens the `PR` tab: a static `3 PR` label, a header of identity and state tokens (`merge`, `sync`, lifecycle), a `checks` list with a rollup, and a newest-first `comments` list merging reviews, inline threads, and issue comments with `outdated`/`resolved` markers.
- `↵`, a click, or `j`/`k` reads the selected comment on the left — a finding's `diff_hunk` as text then its body, a review or comment as prose.
- `o` opens the PR, the selected check, or the selected comment in the browser; `r` refetches; the snapshot also refetches every 30s and on switching to the tab.
- No-PR, `gh`-missing/unauthed, and non-GitHub-remote each render their own empty state; a failed poll never blanks a populated tab.
- `forge` normalization and the sync count are unit-tested against fixture JSON; the tab renders against a live open PR.

## Exit State

The live shape when done — a closed list; anything unnamed is not built. This plan's subset of the specs' end-state; it references their model, never re-enumerating it.

- `src/forge.rs` — shells `gh`, produces the `PrSnapshot` of `forge-host.md` (`number`, `title`, `url`, `state`, `is_draft`, `author`, `base_ref`, `head_ref`, `head_oid`, `merge`, `sync`, `checks`, `comments`), with open-PR resolution, `merge`/`sync` derivation, check normalization + rollup, the three-surface comment merge + bot-latest dedup + newest-first sort, and the typed degraded results.
- `Tab::Pr` in the tab enum; `App` holds the snapshot, the comments cursor, the PR-pane focus, and the 30s PR-poll deadline.
- `render_pr` in `ui.rs` — header tokens, left read-pane (reusing the comment-card and diff-text rendering), right navigator (`checks` then `comments`), the rollup, and the empty/degraded states.
- PR hit-tests in `ui.rs` — a header `open ↗` target, and body targets to select a comment or open a check/comment.
- `src/browser.rs` — an `open`/`xdg-open` probe, mirroring the clipboard-tool probe in `export.rs`.
- Key routing for `Tab::Pr` — `3` switch, `↵`/`j`/`k` select-read, `o` open, `r` refetch, `tab` focus; `c`/`v`/`d`/`e`/`s` inert.

## Specs Touched

| Spec | What this plan realizes | At the gate |
| --- | --- | --- |
| `forge-host.md` | the whole GitHub-reading concern | Draft → Current |
| `tui.md` | the `PR` tab section (the rest is unchanged) | Draft → Current |
| `overview.md` | the read-only PR-tab scope and invariant | Draft → Current |

## Out of Scope

Orientation only — near end-state surface deliberately absent.

- Agent routing, merge, resolve, and update-branch — the tab is read-only (`overview.md`).
- Severity, check durations, the required-check flag, and `reviewDecision` — cut in `forge-host.md`.
- Threaded reply bodies (only `reply_count`) and any cross-session persistence.
- A second forge — GitHub via `gh` only.

## Likely Files

- `src/forge.rs` — created: `gh` reads and `PrSnapshot` normalization.
- `src/browser.rs` — created: the open-in-browser probe.
- `src/git.rs` — touched: an `unpushed` count (`git rev-list --count <head_oid>..HEAD`) for `sync`.
- `src/app.rs` — touched: `Tab::Pr`, the PR state fields, `set_tab` fetch, inert-key guards.
- `src/ui.rs` — touched: `render` dispatch, `render_pr`, and the PR hit-tests.
- `src/lib.rs` — touched: the `3` key, the 30s PR poll and refetch-on-switch, PR key/mouse routing.
- `src/lib.rs` module list — touched: register `forge` and `browser`.

## Execution Plan

1. Build `forge.rs`: the `gh` calls, parse into `PrSnapshot`, the `merge`/`sync` derivation, check normalization, and the three-surface comment merge + dedup + sort; unit-test the normalization on fixture JSON.
2. Add the `git.rs` `unpushed` helper; unit-test it.
3. Extend `App`: `Tab::Pr`, the snapshot and cursors, `set_tab(Pr)` triggers a fetch, inert-key guards; test the tab-state transitions.
4. Add `render_pr` and the PR hit-tests in `ui.rs`, reusing the comment-card and diff-text rendering for the read pane.
5. Wire `lib.rs`: the `3` switch, the 30s PR poll + refetch-on-switch + `r`, the select/open/focus keys and clicks, and `browser.rs` for `o`.
6. Run against a live open PR and each degraded state.

## Verification

- **Done:** `cargo test` passes the `forge` and `git` unit tests; running the sidebar on a repo with an open PR shows the tab, select reads, `o` opens, `r` and the 30s poll refetch, and the degraded states render.
- **Tight:** the diff equals Exit State — no write path, no severity/durations/required/`reviewDecision`, no persistence, no second forge.
- **Invariants upheld:** never writes GitHub — `forge.rs` issues only read subcommands (`gh pr list/view`, `gh api` GET, `gh api graphql` queries); the `PR` tab authors nothing — `c`/`v`/`d`/`e`/`s` are inert there; a failed poll never blanks a populated tab — the PR reload keeps the last snapshot and reports to the status line, as the worktree reload does in `event_loop`; the crate forbids `unsafe`.

## Replan Triggers

- If the user's `gh` token lacks the scope for `mergeStateStatus` or `reviewThreads`, fall back to `gh pr view --json` fields and degrade the missing token to `checking`/omit; record it in `forge-host.md`.
- If merging the three comment surfaces by `created_at` mis-orders (clock skew, edit-in-place), revisit the sort key and record it in `forge-host.md`.

## Replan Log

- 2026-06-27: initial plan from the approved Draft specs.
- 2026-06-27: dropped the tab-label trouble dot — glancing it from another tab needs always-on background polling, which `tui.md` lists as a v1 non-goal (polls stay on the draw path). The label is static `3 PR`; trouble shows in the header state tokens while on the tab. `tui.md` updated to match.
