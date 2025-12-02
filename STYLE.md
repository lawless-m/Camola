# Coding Style & Project Structure

Conventions for the webcam-fx Rust project.

## Project Layout

```
webcam-fx/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI arg parsing
│   ├── lib.rs               # Re-exports, top-level types
│   ├── capture/
│   │   ├── mod.rs
│   │   └── v4l.rs           # Webcam capture via v4l
│   ├── output/
│   │   ├── mod.rs
│   │   └── loopback.rs      # v4l2loopback output
│   ├── segmentation/
│   │   ├── mod.rs           # SegmentationModel trait, re-exports
│   │   ├── traits.rs        # Trait definition
│   │   ├── rvm.rs           # RobustVideoMatting implementation
│   │   ├── modnet.rs        # MODNet implementation (fallback)
│   │   └── preprocess.rs    # Shared image prep utilities
│   ├── effects/
│   │   ├── mod.rs
│   │   ├── background.rs    # Static/video background replacement
│   │   ├── plasma.rs        # Procedural plasma effect
│   │   ├── trails.rs        # Ghosting/echo effect (frozen frame history)
│   │   ├── glitch.rs        # Digital corruption, RGB split, displacement
│   │   └── compositor.rs    # Alpha blending, layering
│   └── pipeline/
│       ├── mod.rs
│       └── runner.rs        # Main frame loop orchestration
├── models/                   # ONNX model files (git-ignored, downloaded)
├── assets/                   # Background images/videos
└── docs/
    ├── README.md
    ├── MODELS.md
    ├── STYLE.md
    └── PLAN.md
```

## Dependencies

Core crates we'll use:

```toml
[dependencies]
# Camera capture
nokhwa = { version = "0.10", features = ["input-v4l"] }
# or
v4l = "0.14"

# ONNX inference
ort = { version = "2.0", features = ["cuda", "tensorrt"] }

# Image handling
image = "0.25"
imageproc = "0.24"

# Linear algebra / tensor ops
ndarray = "0.15"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# CLI
clap = { version = "4.0", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Rust Style

### General

- **Edition:** 2021
- **Format:** Run `cargo fmt` before committing
- **Lints:** Enable `#![warn(clippy::all)]` in `lib.rs`

### Naming

- Types: `PascalCase`
- Functions, methods, variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Error Handling

Use `anyhow` for application code (main, CLI, pipeline orchestration). Use `thiserror` for library-style modules that might be reused.

```rust
// In segmentation/model.rs
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("failed to load model: {0}")]
    Load(#[from] ort::Error),
    #[error("inference failed: {0}")]
    Inference(String),
}

// In main.rs / pipeline
use anyhow::{Context, Result};

fn run() -> Result<()> {
    let model = Model::load(path).context("loading segmentation model")?;
    // ...
}
```

### Struct Design

Prefer builders or `new()` with sensible defaults. Avoid large argument lists.

```rust
pub struct PipelineConfig {
    pub capture_device: u32,
    pub output_device: String,
    pub model_path: PathBuf,
    pub target_fps: u32,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            capture_device: 0,
            output_device: "/dev/video10".into(),
            model_path: "models/modnet.onnx".into(),
            target_fps: 30,
        }
    }
}
```

### Concurrency

The pipeline will likely be single-threaded per frame (GPU work is async anyway). If we need parallelism later (e.g., separate capture/inference/output threads), use channels (`std::sync::mpsc` or `crossbeam`).

### Comments

- Doc comments (`///`) on public items
- Inline comments for non-obvious logic
- No comments for self-explanatory code

### Testing

- Unit tests in the same file (`#[cfg(test)] mod tests`)
- Integration tests in `tests/` directory
- Real hardware tests probably manual — hard to unit test camera/GPU

## Performance Considerations

### Hot Path

The main loop is:
1. Grab frame
2. Preprocess
3. Infer
4. Composite
5. Output

Avoid allocations in this loop. Reuse buffers.

```rust
// Good: pre-allocate, reuse
let mut frame_buffer = vec![0u8; width * height * 3];
let mut matte_buffer = vec![0f32; model_width * model_height];

loop {
    capture.read_into(&mut frame_buffer)?;
    // ... reuse matte_buffer ...
}
```

### GPU Memory

Keep tensors on GPU as much as possible. Minimise CPU↔GPU transfers. The inference output can stay on GPU if compositing also happens on GPU (though for v1, CPU compositing is fine).

### Profiling

Use `tracing` spans to measure where time goes:

```rust
let _span = tracing::info_span!("inference").entered();
model.run(&input)?;
```

## Build Configuration

### Debug vs Release

- Debug builds for development (fast compile, slow run)
- Release builds for actual use: `cargo build --release`

### Feature Flags

Consider feature flags for optional backends:

```toml
[features]
default = ["cuda"]
cuda = ["ort/cuda"]
tensorrt = ["ort/tensorrt"]
cpu-only = []
```

## Git Conventions

- Commit messages: imperative mood ("Add plasma effect", not "Added plasma effect")
- Keep commits focused — one logical change per commit
- Branch naming: `feature/plasma-effect`, `fix/segmentation-crash`
- Don't commit model files or large assets — use git-lfs or document where to download
