# Camola

Real-time webcam segmentation and background effects for video calls (Teams, Zoom, etc.) on Linux.

## Project Status

**Milestone 1: Passthrough Pipeline** ✅ Complete
**Milestone 2: Segmentation Integration** ✅ Code Complete

The basic pipeline and segmentation infrastructure are implemented:
- Webcam capture using `nokhwa`
- v4l2loopback virtual camera output
- Main loop with frame timing and FPS stats
- CLI argument parsing
- RobustVideoMatting segmentation model support
- Matte visualization (grayscale silhouette output)

## What's Implemented

### Capture Module (`src/capture/`)
- `CaptureSource` trait for camera abstraction
- `WebcamCapture` implementation using nokhwa
- Supports configurable resolution

### Output Module (`src/output/`)
- `OutputSink` trait for output abstraction
- `V4L2Output` implementation for v4l2loopback
- RGB to YUYV color space conversion
- Automatic frame resizing

### Segmentation Module (`src/segmentation/`)
- `SegmentationModel` trait for model abstraction
- Preprocessor with resize, normalize, NCHW conversion
- RobustVideoMatting implementation with recurrent state
- Postprocessing to resize matte back to frame dimensions
- Matte-to-RGB conversion for visualization

### Main Pipeline (`src/main.rs`)
- CLI with configurable input/output devices
- Optional segmentation with `--model` flag
- Matte visualization with `--show-matte` flag
- Frame timing for capture + segmentation + output
- Frame rate limiting
- Logging with tracing

## Prerequisites

### v4l2loopback

Install v4l2loopback kernel module:

```bash
sudo apt install v4l2loopback-dkms v4l2loopback-utils
```

Load the module:

```bash
sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="Camola" exclusive_caps=1
```

### ONNX Runtime (for segmentation)

The segmentation features require ONNX Runtime with CUDA support. The build process will attempt to download it automatically, but may require manual installation in some environments:

```bash
# Download ONNX Runtime 1.22.0 with CUDA 12 support
# Extract to a location and set ORT_LIB_LOCATION environment variable
export ORT_LIB_LOCATION=/path/to/onnxruntime/lib
```

## Building

```bash
cargo build --release
```

## Running

### Passthrough Mode (no segmentation)

```bash
# Default: captures from /dev/video0, outputs to /dev/video10
cargo run --release
```

### With Segmentation

```bash
# With RVM model, show matte visualization
cargo run --release -- \
  --model models/rvm_mobilenetv3_fp32.onnx \
  --show-matte

# With segmentation but show original video (for future compositing)
cargo run --release -- \
  --model models/rvm_mobilenetv3_fp32.onnx

# Full configuration
cargo run --release -- \
  --input-device 0 \
  --output-device /dev/video10 \
  --capture-width 1920 \
  --capture-height 1080 \
  --output-width 1280 \
  --output-height 720 \
  --fps 30 \
  --model models/rvm_mobilenetv3_fp32.onnx \
  --show-matte \
  --debug
```

Then select `/dev/video10` in Teams/Zoom/etc.

### Getting the RVM Model

Download the RobustVideoMatting ONNX model:

```bash
cd models
wget https://github.com/PeterL1n/RobustVideoMatting/releases/download/v1.0.0/rvm_mobilenetv3_fp32.onnx
```

Or export from PyTorch (see RVM repository for details).

## Next Steps

### Milestone 3: Background Replacement
- Implement alpha blending (foreground × alpha + background × (1 - alpha))
- Load static images as backgrounds
- Load video files as backgrounds (loop playback)
- CLI flags to select background source

### Future Milestones
- Trails/ghosting effect
- Plasma background
- Glitch effects
- Cyberspace (80s vector graphics)
- Face mesh wireframe

See [../PLAN.md](../PLAN.md) for full roadmap.

## Project Structure

```
camola/
├── src/
│   ├── main.rs              # Entry point, CLI, main loop
│   ├── capture/             # Webcam input
│   │   ├── mod.rs
│   │   └── v4l_capture.rs
│   ├── output/              # v4l2loopback output
│   │   ├── mod.rs
│   │   └── loopback.rs
│   └── segmentation/        # Segmentation models
│       ├── mod.rs           # Model factory
│       ├── types.rs         # SegmentationModel trait
│       ├── preprocess.rs    # Image preprocessing
│       └── rvm.rs           # RobustVideoMatting implementation
├── models/                  # ONNX model files (download separately)
└── assets/                  # Background images/videos
```

## Notes

- Passthrough mode works without ONNX Runtime
- Segmentation requires ONNX Runtime 1.22.0+ with CUDA 12 support
- RVM model provides temporal consistency (no flickering between frames)
- Frame timing shows capture + segmentation + output performance
- RGB→YUYV conversion happens on CPU (acceptable for now)
- Segmentation inference runs on GPU via CUDA/TensorRT
