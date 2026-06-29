# Milestone 02: Theme set

**Plan:** ./main.md · **Specs:** ../../../specs/ — the living reference this plan delivers.

## Goal

Every palette in the curated set selectable and rendering coherently, on the M1 engine.

## Why This Comes Next

M1 proved the anchor→derive model on two themes. This fills in the rest: pure repetition for the `two-face`-paired themes, plus a small generalization (bundled `.tmTheme` bytes) for the four `two-face` lacks.

## Entry State

Builds on `01-palette-engine.md`: the `Palette`/`Theme` model, derivation helpers, config + `--theme` selection, and two themes (`catppuccin`, `catppuccin-latte`).

## Definition of Done

- 18 named themes resolve, each rendering chrome + syntax coherently; every derived theme's diff fills clear the contrast floor.
- `tokyo-night`, `tokyo-night-day`, `rose-pine`, `rose-pine-dawn` render with their bundled syntax themes.
- A bundled `.tmTheme` that fails to parse degrades to plain spans, not a crash (`theme.md` failure semantics).
- Bundled `.tmTheme` licenses attributed in `README`.
- Tests pass: all themes resolve, carry the right appearance, and keep fills legible.

## Exit State

A **closed** list. Realizes `theme.md`'s full committed set.

- `theme.rs`: the registry holds 18 themes — `catppuccin`, `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`, `dracula`, `nord`, `gruvbox`, `gruvbox-light`, `one-dark`, `one-light`, `solarized`, `solarized-light`, `github-light`, `monokai`, `tokyo-night`, `tokyo-night-day`, `rose-pine`, `rose-pine-dawn` — via the `derived` (two-face) and `bundled` (`.tmTheme` bytes) helpers.
- `SyntaxChoice::Bundled(&'static [u8])` replaces `BundledMocha`; `catppuccin` uses it for the Mocha asset.
- `assets/`: `tokyo-night.tmTheme`, `tokyo-night-day.tmTheme`, `rose-pine.tmTheme`, `rose-pine-dawn.tmTheme` vendored.
- `highlight.rs`: `Highlighter` holds `Option<Theme>`, falling back to plain spans when a bundled theme fails to parse.
- `README`: license attribution for the bundled `.tmTheme` files.

## Out of Scope

- `kanagawa`, `kanagawa-lotus`, `vesper`, dark `github`, `everforest`, `ayu` → roadmap (need bundled `.tmTheme`s `two-face` lacks).
- OSC light/dark autodetect, `terminal` mode, `auto_switch`, herdr-config mirroring → `theme.md` Non-goals.

## Likely Files

- `src/theme.rs` — touched: 16 new registry entries, `derived`/`bundled` helpers, anchors, bundled-asset consts.
- `src/highlight.rs` — touched: `Option<Theme>`, `Bundled(bytes)` parsing, plain-span fallback.
- `assets/*.tmTheme` — 4 added.
- `src/diff.rs` — touched: test call sites use the resolved Mocha syntax.
- `README.md` — touched: bundled-theme license attribution.

## Execution Plan

1. Add the 8 two-face-paired themes (anchors + `derived`).
2. Add the 4 free non-herdr extras (`github-light`, `monokai`, Frappé, Macchiato).
3. Generalize `SyntaxChoice` to `Bundled(bytes)`; degrade to plain spans on parse failure.
4. Vendor the 4 `.tmTheme` files; add the 4 bundled themes.
5. Attribute licenses in `README`.
6. Extend tests to cover all 18 (resolve, appearance, fill legibility).

## Verification

- **Done:** `cargo test` green incl. the all-themes legibility/appearance tests; `--theme rose-pine` / `tokyo-night` render in the app.
- **Tight:** the diff equals Exit State — 18 registry entries, 4 vendored assets, no deferred theme present.
- **Invariants upheld:** chrome↔syntax coherence (one selection drives both); transparent canvas (unchanged); Mocha pixel-identical (the M1 render snapshot still passes) — per `theme.md`.

## Replan Triggers

- If a vendored `.tmTheme` parses wrong (off token colors), then re-source it or pin slots; covered by the plain-span fallback meanwhile.

## Replan Log

- 2026-06-29: created when M2 reshaped to the curated set (see `main.md`).
