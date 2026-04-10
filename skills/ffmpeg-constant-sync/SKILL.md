---
name: ffmpeg-constant-sync
description: Use when auditing or updating hand-written FFmpeg constants, macro aliases, and version-gated feature flags in Rust bindings. Trigger for tasks like comparing `pixfmt.h` / `channel_layout.h` / `error.h` against `src/avutil/*.rs`, checking whether FFmpeg 8.1 added constants, determining the first FFmpeg release that introduced a symbol, or adding `ffmpegX_Y` feature gates in `Cargo.toml`.
---

# FFmpeg Constant Sync

Audit upstream FFmpeg header macros against this repository's hand-written Rust constants, then gate each symbol by the earliest release tag that actually contains it.

Prefer release tags such as `n7.1`, `n8.0`, `n8.1` when answering questions about `.0` or `.1` releases. Only use release branches to inspect current maintenance-line state; branch heads may contain later backports.

## Use This Skill For

- Checking whether a specific FFmpeg release added any new constants.
- Syncing `src/avutil/pixfmt.rs`, `src/avutil/channel_layout.rs`, or `src/avutil/error.rs`.
- Deciding whether a symbol belongs behind `ffmpeg6`, `ffmpeg6_1`, `ffmpeg7_1`, `ffmpeg8`, `ffmpeg8_1`, etc.
- Adding missing feature boundaries in `Cargo.toml`.
- Verifying that Rust-exported constants match upstream macro names.

## Files To Audit

- `Cargo.toml`
- `src/avutil/pixfmt.rs`
- `src/avutil/channel_layout.rs`
- `src/avutil/error.rs`
- Sometimes `src/lib.rs` and `src/avutil/mod.rs` if a whole module is feature-gated

Primary upstream headers:

- `libavutil/pixfmt.h`
- `libavutil/channel_layout.h`
- `libavutil/error.h`

## Workflow

1. Identify the local hand-written constants and their current `#[cfg(feature = ...)]` gates.
2. Compare upstream header macros to local Rust exports.
3. For every disputed symbol, find the earliest FFmpeg release tag that contains it.
4. Update `Cargo.toml` feature boundaries if an intermediate version boundary is missing.
5. Gate symbols by the earliest release that contains them.
6. Verify with `DOCS_RS=1 cargo check` across the relevant feature chain.

## Rules

- Use release tags for release questions:
  - Good: `n7.1`, `n8.0`, `n8.1`
  - Less reliable for `.0` / `.1` questions: `release/8.0`, `release/8.1`
- Gate by first availability, not by the first version this repository happened to support.
- If a later feature inherits an earlier one, prefer the narrowest correct gate:
  - Example: if a symbol exists since FFmpeg 6.0, use `ffmpeg6`, not `any(feature = "ffmpeg6", feature = "ffmpeg7")`
- When checking whether `8.1` added constants, diff `n8.0` vs `n8.1` directly.
- Distinguish:
  - Missing symbol entirely
  - Symbol present but feature gate too new
  - Symbol present but feature gate too broad or imprecise
  - Symbol already provided by generated `binding.rs`, so do not duplicate it in hand-written code

## Version Mapping

Current repository feature boundaries should follow this pattern when needed:

- `ffmpeg5`
- `ffmpeg6`
- `ffmpeg6_1`
- `ffmpeg7`
- `ffmpeg7_1`
- `ffmpeg8`
- `ffmpeg8_1`

If the repo is missing a needed boundary, add it to `Cargo.toml` before moving symbol gates.

## Verification

- Run `DOCS_RS=1 cargo check --features <feature>` for each affected boundary.
- At minimum, check the earliest feature that should expose a symbol and the previous feature that should not.
- If you need exact command patterns, read `references/tag-audit-commands.md`.

## Output Expectations

When reporting results:

- State whether the release added new constants or not.
- Name the exact tag pair used for the comparison.
- Call out symbols whose gates moved.
- Separate “missing constant” from “wrong version gate”.
