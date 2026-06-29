---
Status: Current
Created: 2026-06-29
Last edited: 2026-06-29
---

# Theme

The color model: the named palettes reviewr renders in, how each palette fills its slots from a few anchor colors, and how a theme is selected and paired with its syntax colors.

## Overview

A theme is **a handful of anchor colors plus a paired syntax theme**; every other UI color is derived from the anchors, so a theme costs ~8 values, not ~20. The same theme colors both the chrome (file list, footer, header, PR chip, comment cards) and the diff, so the two never clash.

A user selects a theme by name in reviewr's config file, using the **same name they use in herdr**:

```toml
# $HERDR_PLUGIN_CONFIG_DIR/config.toml
theme = "tokyo-night"
```

A theme names its anchors; the rest is computed. The `catppuccin` (Mocha) theme, the default:

| anchor | role | example (`catppuccin`) |
|--------|------|------------------------|
| `base` | the derivation reference and diff-fill background | `#1e1e2e` |
| `text` | primary foreground | `#cdd6f4` |
| `red` | deletion accent | `#f38ba8` |
| `green` | insertion accent | `#a6e3a1` |
| `yellow` | warning / draft accent | `#f9e2af` |
| `peach` | secondary accent | `#fab387` |
| `mauve` | merged / keyword accent | `#cba6f7` |
| `lavender` | link / focus accent | `#b4befe` |

Every other slot is derived from those:

| derived slot | derived from | use |
|--------------|--------------|-----|
| `surface0/1/2` | `base`, lightened in steps (darkened for light themes) | fold, selection, cursor fills |
| `overlay0/1` | `base`, stepped further along the surface ramp | borders, dim chrome |
| `subtext0` | `text`, dimmed toward `base` | secondary text |
| `del_bg` / `ins_bg` | `red` / `green` blended over `base` | deletion / insertion row tint |
| `emph_del_bg` / `emph_ins_bg` | `red` / `green`, a stronger blend over `base` | word-emphasis fill |

The end-state theme set covers herdr's popular named palettes plus a few popular non-herdr ones whose syntax `two-face` already carries:

- Catppuccin: `catppuccin` (Mocha), `catppuccin-latte`, `catppuccin-frappe`, `catppuccin-macchiato`.
- Dark: `dracula`, `nord`, `gruvbox`, `one-dark`, `solarized`, `monokai`, `tokyo-night`, `rose-pine`.
- Light: `gruvbox-light`, `one-light`, `solarized-light`, `github-light`, `tokyo-night-day`, `rose-pine-dawn`.

A name herdr ships that is not in this set — `kanagawa`, `kanagawa-lotus`, `vesper`, `terminal`, and a dark `github` — resolves to the default until added (Non-goals). Most themes pair with a `two-face` syntax theme; `catppuccin` (Mocha), `tokyo-night`, `tokyo-night-day`, `rose-pine`, and `rose-pine-dawn` pair with a bundled `.tmTheme` `two-face` lacks.

## Behavior

### Palette derivation

- A theme lists its anchors; derivation computes every other slot from them, so one source defines the whole palette.
- One theme — `catppuccin` — instead pins its whole palette as a literal, to stay byte-identical to the pre-theming colors; there is no per-slot pin.
- The cursor/selection/fold ramp is `surface2` > `surface1` > `surface0` — the cursor is the strongest-contrast fill, a fold the faintest (for a light theme the ramp steps toward black, not white).
- Each theme declares its `appearance` (light or dark), which sets the derivation direction: dark themes lighten `base` for surfaces, light themes darken it.
- The `catppuccin` theme's slots resolve to the canonical Catppuccin Mocha values, so it renders as a faithful Mocha.

### Diff-fill legibility

- `del_bg` / `ins_bg` blend the `red` / `green` accent over `base`; `emph_del_bg` / `emph_ins_bg` blend more strongly for the changed words.
- The blend ratio starts higher on a dark base than a light one.
- It then steps down until the row's `text` keeps a minimum contrast ratio against the fill, so code on a fill stays legible on any base.

### Chrome and syntax pairing

- Selecting a theme sets both the chrome palette and the syntax-highlighting theme; they always come from the same theme and never desync.
- Each theme pairs with a `syntect` theme — from the `two-face` set, or a bundled `.tmTheme` as `catppuccin` uses. Adding a palette pairs it with its syntax theme.
- Syntax spans contribute foreground token colors only; the pane background stays transparent, so the diff sits on the terminal's own background.
- A theme reads correctly only when its `appearance` matches the terminal's light or dark, since the canvas is the terminal's; the user picks a theme matching their terminal.

### Theme selection

- Precedence is `--theme <name>` (flag) over `theme` (config) over the default `catppuccin`.
- The config file is `$HERDR_PLUGIN_CONFIG_DIR/config.toml`, re-read on refresh, so editing `theme` and refreshing re-themes without relaunching the pane.
- Theme names match herdr's where both ship a palette, so the value a user copies from their herdr config resolves to the same palette.
- A herdr name not yet in the set (`kanagawa`, `vesper`, `terminal`, a dark `github`) resolves to the default until added.
- Standalone (no `HERDR_PLUGIN_CONFIG_DIR`), reviewr reads no config file and relies on `--theme` and the default.

## Failure semantics

reviewr only ever reads the config file; it never writes it, so concurrent sidebars and repeated refresh re-reads are safe and need no coordination.

- An unknown or not-yet-supported theme name, from the flag or the config, resolves to the default `catppuccin` and is logged; the UI shows no error and never half-applies a palette.
- A missing or unparseable config file resolves to the default theme; a later refresh that finds it valid applies it.
- A theme whose paired syntax theme fails to load still renders its chrome; syntax falls back to plain foreground spans, as `diff-view.md` defines.
- Re-reading config on refresh is idempotent: an unchanged `theme` reuses the built palette; a changed value rebuilds it.

## Non-goals

- No new UI affordance — theming changes only colors; the theme is a rarely-changed config value, so it adds no key, switcher, indicator, or status surface.
- No reading of herdr's own `[theme]` to mirror it — matching names let a user keep the two in sync by hand without coupling to herdr's config.
- No `terminal` theme deriving chrome from the live terminal palette (OSC 4/10/11) — roadmap.
- No OSC light/dark auto-detection of a default — the default is `catppuccin` regardless of terminal appearance; roadmap.
- No `auto_switch`-style light/dark pairing — light and dark are separate named themes (`catppuccin` and `catppuccin-latte`).
- No custom or user-defined palettes — only the named themes.
- No `kanagawa`, `kanagawa-lotus`, `vesper`, or dark `github` yet — each needs a bundled `.tmTheme` `two-face` lacks; roadmap.
- No colorblind secondary cue for add/remove — still red/green; roadmap.
- No config keys beyond `theme` — `--poll`, `--base`, `--wrap` stay CLI-only; the reintroduced config file does not restore the removed `keep` list.

## Decisions

- Hybrid anchors-plus-derive, not fully hand-listed or fully syntax-derived — listing ~8 anchors keeps accent fidelity while deriving surfaces and diff fills keeps each theme cheap and coherent on any base. Rejected: hand-listing every slot; deriving the whole chrome from the `.tmTheme`, which carries no named accents, so the chrome goes flat.
- A curated set, not herdr's list verbatim — covering herdr's popular names plus popular non-herdr palettes whose syntax `two-face` already ships (`github-light`, `monokai`, Catppuccin Frappé/Macchiato) maximizes coverage per unit of work; herdr's niche tail that needs a bundled `.tmTheme` (`kanagawa`, `vesper`) waits. Rejected: matching herdr's 17 exactly, which bundles niche assets before popular free ones.
- Theme names match herdr's where shared — a herdr user's config value resolves to the same palette, and the shared vocabulary keeps the design open to later herdr-following. Rejected: reviewr-specific names.
- Selection by config and `--theme`, colors only — the installed pane takes no CLI args and the theme changes rarely, so a config value suffices and the UI gains no affordance. Rejected: an in-TUI selector or any new indicator.
- reviewr does not read herdr's config — mirroring herdr's active theme would couple reviewr to a third-party config schema it does not own; matching names give hand-sync instead. Rejected: reading `~/.config/herdr/config.toml`.
- One theme sets chrome and syntax together — a single selection makes them coherent by construction; the prior `--theme` selected only the syntect theme while the chrome stayed Mocha. Rejected: separate chrome and syntax selectors.
- Default `catppuccin`, not terminal-derived — a deterministic default without an OSC probe; auto light/dark is roadmap. Rejected: probing the terminal for a light/dark default now.

## Open decisions

- None.

## Related specs

- `./diff-view.md`
- `./tui.md`
- `./herdr-host.md`
