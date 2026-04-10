# Tag Audit Commands

Use these patterns when auditing FFmpeg constants.

## Diff two release tags

```bash
diff -u \
  <(git -C /path/to/ffmpeg show n8.0:libavutil/pixfmt.h | rg '^#define AV_PIX_FMT_') \
  <(git -C /path/to/ffmpeg show n8.1:libavutil/pixfmt.h | rg '^#define AV_PIX_FMT_')
```

Repeat with:

- `libavutil/channel_layout.h`
- `libavutil/error.h`

Useful filters:

- `^#define AV_PIX_FMT_`
- `^#define AV_CH_`
- `^#define AV_CH_LAYOUT_`
- `^#define AV_CHANNEL_LAYOUT_`
- `^#define AVERROR_`

## Find the first release tag containing a symbol

```bash
for tag in n5.0 n6.0 n6.1 n7.0 n7.1 n8.0 n8.1; do
  if git -C /path/to/ffmpeg show "$tag:libavutil/pixfmt.h" | rg -q '^#define AV_PIX_FMT_GRAY32\b'; then
    echo "$tag"
    break
  fi
done
```

Switch files by symbol family:

- `AV_PIX_FMT_*` -> `libavutil/pixfmt.h`
- `AV_CH_*`, `AV_CH_LAYOUT_*`, `AV_CHANNEL_LAYOUT_*` -> `libavutil/channel_layout.h`
- `AVERROR_*` -> `libavutil/error.h`

## Compare local hand-written aliases with upstream macros

```bash
comm -23 \
  <(git -C /path/to/ffmpeg show n8.1:libavutil/pixfmt.h | rg '^#define AV_PIX_FMT_[A-Z0-9_]+\s+AV_PIX_FMT_NE' | sed -E 's/^#define (AV_PIX_FMT_[A-Z0-9_]+).*/\1/' | sort -u) \
  <(rg '^AV_PIX_FMT_NE!\(' src/avutil/pixfmt.rs | sed -E 's/^AV_PIX_FMT_NE!\((AV_PIX_FMT_[A-Z0-9_]+).*/\1/' | sort -u)
```

## Compile checks

```bash
DOCS_RS=1 cargo check --features ffmpeg6
DOCS_RS=1 cargo check --features ffmpeg6_1
DOCS_RS=1 cargo check --features ffmpeg7
DOCS_RS=1 cargo check --features ffmpeg7_1
DOCS_RS=1 cargo check --features ffmpeg8
DOCS_RS=1 cargo check --features ffmpeg8_1
```

Use the smallest affected subset when possible, but include the boundary before and after the symbol’s first supported version.
