# Segmentation Models

This document covers the neural network models we can use for separating you from your background.

## What We Need

A model that takes a video frame and outputs an **alpha matte** — a grayscale image where white = you, black = background, and grey values = partial transparency (hair edges, motion blur, etc.). This is better than a hard binary mask because it gives natural-looking edges.

## Model Options

### MODNet (Recommended Starting Point)

**What it is:** Portrait matting model from Zheng et al., specifically designed for video conferencing use cases.

**Why it's good:**
- Trained specifically for webcam/portrait scenarios
- Outputs true alpha matte with soft edges
- Multiple model sizes available
- Good documentation, easy to export to ONNX

**Performance:** ~30-40fps on GPU at 512×512 input, faster with TensorRT

**Source:** https://github.com/ZHKKKe/MODNet

### RobustVideoMatting (RVM) — Primary Target

**What it is:** Video-focused matting model with temporal consistency built in.

**Why it's good:**
- Designed for video, not just single images
- Has internal recurrence — uses previous frame info to reduce flickering
- Very smooth results, edges don't "dance" between frames
- High quality alpha output (true matting, not segmentation)

**How it works:**
RVM maintains hidden state tensors (recurrent memory) between frames. You feed in the current frame plus the previous hidden states, and it outputs the alpha matte plus updated hidden states for the next frame. This temporal awareness is why it's so stable.

```
Frame N + Hidden State N-1  →  RVM  →  Matte N + Hidden State N
```

On scene cuts or camera switches, you reset the hidden state to zeros.

**Tradeoff:** Slightly more complex to integrate because of the recurrent state, but our trait abstraction handles this cleanly.

**Source:** https://github.com/PeterL1n/RobustVideoMatting

### MediaPipe Selfie Segmentation

**What it is:** Google's lightweight segmentation, used in Google Meet.

**Why it's good:**
- Very fast
- Well-documented
- Easy to get running

**Tradeoff:** Outputs binary mask, not alpha matte. Edges are blockier. Fine for "rough and ready" but not as polished.

**Source:** https://google.github.io/mediapipe/solutions/selfie_segmentation.html

### PP-HumanSeg (PaddlePaddle)

**What it is:** Baidu's human segmentation suite.

**Why it's good:**
- Multiple model sizes (lite, server, etc.)
- Good accuracy
- Can export to ONNX

**Tradeoff:** Less community support in Rust ecosystem

**Source:** https://github.com/PaddlePaddle/PaddleSeg

## Architecture: Model Neutrality

We'll design the segmentation layer as a trait, so any model can be swapped in without touching the rest of the pipeline.

```rust
pub trait SegmentationModel {
    /// Process a frame, return alpha matte (0.0 = background, 1.0 = foreground)
    fn segment(&mut self, frame: &Frame) -> Result<Matte>;
    
    /// Some models (like RVM) maintain internal state between frames
    fn reset_state(&mut self) {}
    
    /// Model's preferred input dimensions
    fn input_size(&self) -> (u32, u32);
}
```

This lets us:
- Start with whichever model is easiest to get running
- Switch models via config/CLI without code changes
- A/B test different models
- Handle stateful models (RVM) and stateless models (MODNet) uniformly

### Target: RobustVideoMatting

RVM is our primary target because:
- **Temporal consistency** — Uses recurrent architecture, previous frame info reduces flicker
- **Designed for video** — Not just single-image inference bolted onto video
- **High quality edges** — True alpha matting, not binary segmentation

The recurrent state is handled naturally by the trait — RVM's implementation keeps its hidden state internally, resets on `reset_state()` (e.g., if you switch cameras or there's a scene cut).

### Fallback: MODNet

MODNet as a simpler alternative:
- Stateless (each frame independent)
- Slightly easier to integrate initially
- Good for testing the pipeline before tackling RVM's state management

## Running Models in Rust

### The `ort` Crate

This is the Rust binding to ONNX Runtime. It handles:
- Loading ONNX models
- Running inference on CPU or GPU
- CUDA and TensorRT execution providers

```toml
[dependencies]
ort = { version = "2.0", features = ["cuda", "tensorrt"] }
```

### Execution Providers (Priority Order)

1. **TensorRT** — Fastest. Compiles the model specifically for your GPU architecture. First run is slow (compilation), subsequent runs are very fast.
2. **CUDA** — Fast. Uses cuDNN under the hood.
3. **CPU** — Fallback. Works but probably won't hit real-time at good quality.

### Input/Output

Typical model I/O:

**Input:** RGB image, normalised to 0-1 or -1 to 1, resized to model's expected dimensions (often 512×512 or 256×256), NCHW format (batch, channels, height, width).

**Output:** Single-channel alpha matte, same spatial dimensions as input, values 0-1.

### Preprocessing Pipeline

1. Capture frame from camera (BGR typically)
2. Convert BGR → RGB
3. Resize to model input size
4. Normalise pixel values
5. Transpose to NCHW
6. Run inference
7. Resize output matte back to original frame size

## Model Acquisition

We'll need to either:
- Download pre-trained ONNX files from model repos
- Export from PyTorch ourselves using `torch.onnx.export()`

Most repos provide ONNX exports or scripts to generate them.

## Performance Targets

For comfortable real-time at 30fps, we have ~33ms per frame budget.

| Stage | Target |
|-------|--------|
| Frame capture | <5ms |
| Preprocessing | <3ms |
| Inference | <15ms |
| Compositing | <5ms |
| Output to v4l2 | <3ms |

With an RTX 3070/4070 and TensorRT, the inference portion should comfortably fit in 10-15ms for MODNet at 512×512.

## Future Options

Once the basic pipeline works, we could explore:
- **Depth estimation models** — fake bokeh/depth-of-field
- **Face landmark models** — for AR overlays or eye gaze correction
- **Super-resolution** — upscale a lower-res webcam
