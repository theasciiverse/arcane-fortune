# Changes from upstream

This is the Asciiverse packaging of [Arcane Fortune](https://github.com/cody2007/arcane_fortune)
by Darin Straus (code AGPL-3.0-or-later, configuration files CC BY-SA 4.0;
see `license.txt`), built from upstream's `v0.4.0` (`205b214`, 19 June 2021),
the terminal edition (ncurses, without the `sdl` feature). Upstream is the
`upstream` remote; the `asciiverse` branch carries the changes below.

## Ours

- **Builds with today's Rust.** The bundled snappy reader's `fail!` macro
  ended in a semicolon that Rust 1.93 rejects in expression position
  (`src/saving/snappy/read.rs`).
- **The terminal build no longer needs SDL.** `sdl2_lib` was always a
  dependency, so linking needed SDL2, SDL2_image and SDL2_ttf even for
  ncurses. It is now optional, pulled in by the `sdl` feature
  (`Cargo.toml`), and the keyboard code takes `KEY_UP` and `KEY_DOWN` from
  the active renderer instead of from `sdl2_lib` (`src/keyboard/mod.rs`).
- **Data beside the binary, from anywhere.** The game reads `config/` and
  `nn/` relative to the working folder. Started elsewhere (by a launcher,
  or from a package's link on PATH), it now moves to its binary's folder
  first when `config/` is there (`src/paths.rs`, `src/main.rs`).
- **Saves in the player's state folder.** Saved games went to `saves/`
  beside the game, which had to exist already (the game would not start
  without it), and the language preference was written into `config/`.
  Both now go to `$XDG_STATE_HOME/arcane-fortune`, else
  `~/.local/state/arcane-fortune` (Linux), `~/Library/Application Support/arcane-fortune`
  (macOS); the saves folder is made when missing. Until a language is
  chosen, the default in `config/` is read (`src/paths.rs`,
  `src/saving/save_game.rs`, `src/config_load/mod.rs`,
  `src/disp/logo_utils.rs`, `src/disp/window/states/open.rs`,
  `src/localization/mod.rs`).
- **How to play it, at the top of readme.txt.** A short note on installing
  and starting it with av; the original text follows unchanged.

## Packaging

- `nn/`, the neural network behind what other civilizations say in their
  embassies (about 38 MB), is not in upstream's git repository, only in
  its v0.4.0 release zip (`arcane_fortune_v0_4_0.zip`, sha256
  `86b7a5643eb40240f0b8f2d3312aad1ef293885e4eb3a795c09cf61b1a17e43e`);
  `asciiverse/fetch-nn.sh` takes it from there, unchanged.
- `.github/workflows/release.yml`: av archives for Linux (static musl,
  built in Alpine, x86-64 and ARM64) and macOS (x86-64 and ARM64), each with
  `config/` and `nn/` beside the binary, plus SHA256SUMS, and .deb, .rpm,
  .apk and Arch packages (installed in `/usr/lib/arcane-fortune`, with
  `/usr/bin/arcane-fortune` a link). No Windows: upstream's Windows edition
  draws with PDCurses on SDL in its own window, not in a terminal.
- `asciiverse/nfpm.yaml`: the system packages.
- `asciiverse/record-preview.sh` and `asciiverse/arcane-fortune.cast`: the
  catalog preview, recorded with hub's `tools/castrec.py`.
