# Agentic Bevy Game Workspace

A Linux-first Rust workspace combining Bevy, Avian2D physics, Leafwing semantic input, asset-loader state flow, headless tests, and an agent-oriented `xtask` CLI.

## Verified Stack

Bevy `0.20.1`, Avian `0.8.0`, Leafwing `0.22.0`, `bevy_asset_loader` `0.28.1`, and `bevy-inspector-egui` `0.38.0`. See `COMPATIBILITY.md` for the compile evidence and feature decisions. Rust `1.99.0` is selected in `rust-toolchain.toml`; the manifest MSRV follows Bevy at `1.97.1`.

## Linux Setup

Install a C/C++ linker toolchain and `pkg-config`, then install Rust with `rustup` so the repository toolchain file is honored.

| Distribution | System packages |
| --- | --- |
| Debian / Ubuntu | `sudo apt install build-essential pkg-config` |
| Fedora | `sudo dnf group install development-tools && sudo dnf install pkgconf-pkg-config` |
| Arch | `sudo pacman -S --needed base-devel pkgconf` |
| openSUSE | `sudo zypper install -t pattern devel_basis && sudo zypper install pkg-config` |

The default and headless builds need no audio or `libudev` development packages. Optional physical gamepad support needs the corresponding development package: `libudev-dev` on Debian/Ubuntu, `systemd-devel` on Fedora/openSUSE, or `systemd` on Arch.

For faster linking, install `clang` plus `lld` or `mold`, then follow the commented instructions in `.cargo/config.toml`. These flags are deliberately opt-in so a fresh clone builds on machines without them.

## Common Commands

```bash
cargo xtask status
cargo xtask verify
cargo xtask verify --game game_template
cargo xtask new my_game --dimension 2d
cargo xtask doc --no-deps --private
```

Run the template windowed:

```bash
cargo run -p game_template
```

Enable diagnostics and Bevy dynamic linking for local development:

```bash
cargo run -p game_template --features dev
```

Enable physical gamepads (after installing `libudev` development files):

```bash
cargo run -p game_template --features gamepad
```

## Headless CI

The integration test uses no X11/Wayland display:

```bash
env -u DISPLAY -u WAYLAND_DISPLAY cargo test -p game_template --test headless_sim
```

GitHub Actions runs formatting, Clippy with warnings denied, tests, scope reporting, documentation, and a release build on Ubuntu.

## Portable Release Builds

```bash
cargo build --release -p game_template
```

The resulting executable is `target/release/game_template`. Default release builds do not use Bevy dynamic linking. Distribute the executable with the game's `assets/` directory and launch it with the asset root as its working directory. Test on the oldest distribution you intend to support; pure Rust dependencies are portable, but the C library ABI baseline is determined by the build host or container.

## Repository Conventions

Commit `Cargo.lock` for reproducible game and CLI builds. Do not commit `target/`, scratch compatibility builds, local IDE settings, secrets, or machine-specific linker choices. Agents begin at `AGENTS.md`; game status and objectives live in each `games/<name>/SCOPE.md`.
