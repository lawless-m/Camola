# webcam-fx

Real-time webcam segmentation and background effects for video calls (Teams, Zoom, etc.) on Linux.

## Project Status

**Milestone 1: Passthrough Pipeline** ✅ Complete

The basic pipeline is implemented and builds successfully:
- Webcam capture using `nokhwa`
- v4l2loopback virtual camera output
- Main loop with frame timing and FPS stats
- CLI argument parsing

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

### Main Pipeline (`src/main.rs`)
- CLI with configurable input/output devices
- Frame timing and FPS monitoring
- Frame rate limiting
- Logging with tracing

## Building

```bash
cargo build --release
```

## Prerequisites

Install v4l2loopback kernel module:

```bash
sudo apt install v4l2loopback-dkms v4l2loopback-utils
```

Load the module:

```bash
sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="WebcamFX" exclusive_caps=1
```

## Running

```bash
# Default: captures from /dev/video0, outputs to /dev/video10
cargo run --release

# Custom devices and resolution
cargo run --release -- \
  --input-device 0 \
  --output-device /dev/video10 \
  --capture-width 1920 \
  --capture-height 1080 \
  --output-width 1280 \
  --output-height 720 \
  --fps 30 \
  --debug
```

Then select `/dev/video10` in Teams/Zoom/etc.

## Next Steps

### Milestone 2: Segmentation Integration
- Define `SegmentationModel` trait
- Add ONNX Runtime (`ort` crate) with CUDA support
- Implement RobustVideoMatting backend
- Add preprocessing pipeline
- Test with greyscale matte output

See [../PLAN.md](../PLAN.md) for full roadmap.

## Project Structure

```
webcam-fx/
├── src/
│   ├── main.rs           # Entry point, CLI, main loop
│   ├── capture/          # Webcam input
│   │   ├── mod.rs
│   │   └── v4l_capture.rs
│   └── output/           # v4l2loopback output
│       ├── mod.rs
│       └── loopback.rs
├── models/               # ONNX models (to be added)
└── assets/               # Background images/videos (to be added)
```

## Notes

- Current implementation is simple passthrough with no processing
- Frame timing shows capture + output performance baseline
- RGB→YUYV conversion happens on CPU (acceptable for now)
- Future milestones will add GPU-accelerated segmentation and effects
