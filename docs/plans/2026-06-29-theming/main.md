# Theming — Delivery Strategy

**Specs:** ../../../specs/ — the living reference this plan delivers (`theme.md`, `diff-view.md`).

## Goal

Make reviewr render in a curated set of named palettes, selected by a config value, replacing the single hardcoded Catppuccin Mocha. Colors only — no new UI affordance, and the pane background stays the terminal's. See `theme.md`.

## Milestone Map

1. **Palette engine** — the `Theme`/`Palette` model, anchor→derived color math, config-file + `--theme` selection, threaded through `ui.rs`; proven with `catppuccin` (Mocha, pixel-identical to today) and `catppuccin-latte` (light). Ended on an **information boundary**: derivation reproduces a canonical palette and stays legible on a light base — confirmed.
2. **Theme set** — the remaining palettes on the proven model: 12 paired with `two-face` syntax, 4 with a bundled `.tmTheme` (`tokyo-night`×2, `rose-pine`×2). 18 themes total. No internal must-stop; completes the contract.

## Current Milestone

`02-theme-set.md` (built; merge gate next)

## Deferred Decisions

- `kanagawa`, `kanagawa-lotus`, `vesper`, dark `github`, and `everforest`/`ayu` — each needs a bundled `.tmTheme` `two-face` lacks; deferred to a later milestone (roadmap in `theme.md`).

## Review Follow-ups (deferred, non-blocking)

From the xhigh end-to-end review — equivalence-preserving polish, no correctness bugs:

- Plain-line fallback fg: `highlight.rs` `DEFAULT_FG` is Mocha's dark fg, used only if a theme's syntax carries no global `foreground` (none of the 18 shipped themes hit this). If a `foreground`-less theme is ever added, derive the fallback from the active `palette.text`.
- Render-context seam: `ui.rs` threads `&Palette` two ways (a `RowLayout` field plus a bare param on ~14 leaves). A small render-context struct would let a future theme-dependent input ride along without re-touching signatures.
- Theme registry as data: `build()` repeats each name as match-key + arg, and the test `NAMED` table re-lists all themes. A `static THEMES: &[ThemeSpec]` would write each name once and let tests iterate it.
- `app.rs` could store the resolved `Theme` (Copy) instead of separate `palette` + `theme_name`, removing a desync surface.

## Replan Log

- 2026-06-29: initial strategy from the approved `theme.md` contract.
- 2026-06-29: M2 reshaped after discovering only 8 of herdr's 15 remaining palettes have `two-face` syntax; the other 7 need bundled `.tmTheme` assets. Committed set changed from "herdr's 17 verbatim" to a curated 18 — herdr's popular names + popular non-herdr palettes free via `two-face` (`github-light`, `monokai`, Frappé, Macchiato), bundling only the popular `tokyo-night`/`rose-pine`. `theme.md` updated to match.
