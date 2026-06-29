# Milestone 01: Palette engine

**Plan:** ./main.md · **Specs:** ../../../specs/ — the living reference this plan delivers.

## Goal

A theme is a resolved `Palette` plus a paired syntax theme, selected by config or flag and rendered across the whole UI. Delivered working for `catppuccin` (Mocha, pixel-identical to today) and `catppuccin-latte` (light).

## Why This Comes Next

It resolves the information boundary before 15 palettes depend on it: whether the anchor→derive model reproduces a canonical palette (Mocha, via pinned slots) and produces a legible light theme (Latte, via derivation + contrast-stepping). It also commits the `Palette` and derivation contract that M2 fills in.

## Entry State

Baseline repo at v0.4.0: a single hardcoded Mocha (`mod cat` plus the `DEL_BG`/`INS_BG`/`EMPH_*`/`CURSOR_BG`/`SEL_BG`/`FOLD_BG` consts in `ui.rs`), `--theme` selecting only a syntect theme, no config file.

## Definition of Done

- `--theme catppuccin-latte` (or `theme = "catppuccin-latte"` in config) renders the whole UI — file list, diff, footer, PR chip, comment cards — in Latte, legibly.
- Default and `catppuccin` render pixel-identical to today's Mocha.
- An unknown or not-yet-supported name, including `terminal`, falls back to `catppuccin` and logs once; no UI error.
- Editing `theme` in the config file and refreshing re-themes without relaunch, unless `--theme` pins it.
- Tests pass: derivation/contrast units, the Mocha render-buffer regression, config precedence and fallback, and chrome↔syntax coherence.

## Exit State

A **closed** list — anything not named here is not built. This realizes `theme.md`'s model and selection with exactly two palettes; the other 15 are M2.

- `src/theme.rs`: `Anchors` (the 8 anchors per `theme.md`), `Appearance` (`Light`/`Dark`), `Palette` (the ~18 resolved `Color` slots `ui.rs` uses), and `resolve(anchors, appearance, pins) -> Palette`.
- Derivation helpers in `theme`: `blend(base, accent, pct)`, `lighten`/`shift_lightness`, `contrast_ratio`, and `readable_tint` (steps the tint down until min contrast).
- A theme registry with **exactly two** entries: `catppuccin` (slots pinned to the current values where derivation would not match) and `catppuccin-latte` (derived), each carrying anchors, pins, and a paired syntect theme.
- `Highlighter` built from a theme's paired syntect theme: `catppuccin` → bundled `assets/Catppuccin Mocha.tmTheme`; `catppuccin-latte` → its `.tmTheme` (bundled asset or `two_face::theme`).
- `config.rs` reads `$HERDR_PLUGIN_CONFIG_DIR/config.toml` and returns its `theme` value; `toml` crate added; no file read when the env var is unset.
- Selection: resolved name = `--theme` else config `theme` else `catppuccin`, applied in `App::set_theme` (now resolves a name to a `Theme`); `reload()` re-reads config and rebuilds only when the active name changes.
- `ui.rs` render functions and the free helpers (`cursor_bg`, `render_row`, `cells_to_spans`, `cell_span`, `kind_color`, `pr_status_chip`, `check_glyph`) take the active `&Palette`; `mod cat` and the diff-fill consts are removed.

## Specs Touched

| Spec | What this milestone realizes | At the gate |
| --- | --- | --- |
| `theme.md` | the model, derivation, selection, and failure semantics — with 2 of 17 palettes | stays Draft → M2 |
| `diff-view.md` | color from the theme, chrome↔syntax coherence, transparent canvas | stays Draft → M2 (held to avoid a Current doc referencing a Draft `theme.md`) |

## Out of Scope

- The 15 remaining palettes → M2.
- OSC light/dark detection, `auto_switch`, the `terminal` mode, herdr-config mirroring, config keys beyond `theme`, any UI affordance → `theme.md` Non-goals (roadmap).

## Likely Files

- `src/theme.rs` — created: anchors, palette, `resolve`, blend/contrast helpers, the 2-entry registry.
- `src/highlight.rs` — touched: build from a theme's paired syntect theme; add the Latte theme.
- `assets/Catppuccin Latte.tmTheme` — added if not sourced from `two-face`.
- `src/config.rs` — touched: add the `$HERDR_PLUGIN_CONFIG_DIR/config.toml` `theme` read.
- `src/app.rs` — touched: hold the resolved `Theme`/`Palette`; `set_theme` resolves a name and no-ops when unchanged; `reload()` re-resolves from config.
- `src/lib.rs` — touched: wire startup resolution and the config path.
- `src/ui.rs` — touched: thread `&Palette`; remove `mod cat` and the diff-fill consts.
- `Cargo.toml` — touched: add `toml`.
- `tests/render.rs`, `tests/app_flow.rs` — touched: Mocha regression snapshot, Latte render, precedence and fallback.

## Execution Plan

1. Add `src/theme.rs` with `Anchors`, `Appearance`, `Palette`, `resolve`, and the blend/lighten/contrast/`readable_tint` helpers; unit-test the helpers.
2. Define the two themes: `catppuccin` (pin slots to the current `mod cat` and diff-fill values), `catppuccin-latte` (anchors plus derived), each naming its paired syntect theme.
3. Rework `Highlighter` to build from a theme's paired syntect theme; locate or bundle the Latte `.tmTheme`.
4. Thread `&Palette` through `ui.rs`; delete `mod cat` and the diff-fill consts.
5. Add the config-file read in `config.rs` (+`toml`); resolve precedence in `App::set_theme`; store the active name and rebuild only on change; re-resolve in `reload()`; log and default on an unknown name.
6. Add tests: Mocha render-buffer regression, Latte render, derivation/contrast units, config precedence and fallback.

## Verification

- **Done:** `cargo run -- --theme catppuccin-latte` → legible Latte; default → Mocha; `cargo test` green including the Mocha snapshot.
- **Tight:** the diff equals Exit State — exactly two registry entries, no OSC/`auto_switch`/`terminal`/extra config keys, `mod cat` and the diff-fill consts gone.
- **Invariants upheld:** chrome↔syntax coherence (test the palette and `Highlighter` resolve from one theme); transparent canvas (render test: context-row cells set no background); Mocha pixel-identical (render-buffer snapshot equals the baseline) — per `theme.md`.

## Replan Triggers

- If derivation can't make Latte legible without pinning most slots, then switch the model to pin-first/derive-few and raise M2's per-theme effort estimate.
- If `two-face` has no usable Latte syntect theme, then bundle a `.tmTheme` asset, as Mocha is.

## Replan Log

- 2026-06-29: initial plan from the approved `theme.md` contract.
