# Implementation Plan

Phased approach to building webcam-fx. Each milestone is a working checkpoint.

## Goal

A Rust application that:
1. Captures webcam video
2. Segments you from the background (using GPU-accelerated neural network)
3. Replaces the background with video or procedural effects
4. Outputs to a virtual camera that Teams/Zoom/etc. can use

## Milestones

### Milestone 1: Passthrough Pipeline

**Objective:** Get frames flowing from webcam to virtual camera with zero processing.

**Tasks:**
- [ ] Set up project structure (`cargo new webcam-fx`)
- [ ] Implement camera capture using `nokhwa` or `v4l` crate
- [ ] Set up v4l2loopback output
- [ ] Write main loop: capture → output
- [ ] Verify Teams/Zoom can see the virtual camera

**Success criteria:** You can select the virtual camera in Teams and see yourself, latency feels acceptable.

**Estimated effort:** 1-2 sessions

---

### Milestone 2: Segmentation Integration

**Objective:** Implement model-neutral segmentation with RobustVideoMatting as primary target.

**Tasks:**
- [ ] Define `SegmentationModel` trait (segment, reset_state, input_size)
- [ ] Add `ort` dependency, configure CUDA execution provider
- [ ] Implement shared preprocessing (resize, normalise, NCHW conversion)
- [ ] Download/export RobustVideoMatting to ONNX format
- [ ] Implement RVM backend (including recurrent state handling)
- [ ] Implement postprocessing (resize matte back to frame size)
- [ ] Debug visualisation: output the matte as greyscale to virtual camera
- [ ] (Optional) Implement MODNet backend as fallback/comparison

**Success criteria:** Virtual camera shows your silhouette as a greyscale matte in real-time, with smooth temporal consistency (no flicker).

**Estimated effort:** 2-3 sessions

---

### Milestone 3: Background Replacement

**Objective:** Composite yourself onto a replacement background.

**Tasks:**
- [ ] Implement alpha blending (foreground × alpha + background × (1 - alpha))
- [ ] Load static image as background
- [ ] Load video file as background (loop playback)
- [ ] Basic CLI flags to select background source

**Success criteria:** You appear composited over a chosen image or video in Teams.

**Estimated effort:** 1-2 sessions

---

### Milestone 4: Effect — Trails

**Objective:** Implement the trails/ghosting effect with variations.

**Spec:** [EFFECT-TRAILS.md](EFFECT-TRAILS.md)

**Tasks:**
- [ ] Define `Effect` trait
- [ ] Implement frame history ring buffer
- [ ] Basic trails compositing with opacity decay
- [ ] Chroma variation (hue shift per trail)
- [ ] Blur variation (progressive blur)
- [ ] Glitch variation (slice, RGB split, flip)
- [ ] CLI flags to select variation and parameters

**Success criteria:** You move, ghostly copies trail behind you. Variations work.

**Estimated effort:** 1-2 sessions

---

### Milestone 5: Effect — Plasma

**Objective:** Classic demoscene plasma background.

**Spec:** [EFFECT-PLASMA.md](EFFECT-PLASMA.md)

**Tasks:**
- [ ] Basic plasma algorithm (multi-layer sine)
- [ ] Palette system with predefined palettes
- [ ] Speed and scale parameters
- [ ] CLI flags for configuration

**Success criteria:** Animated swirling colours behind you.

**Estimated effort:** 1 session

---

### Milestone 6: Effect — Glitch

**Objective:** Digital corruption effects.

**Spec:** [EFFECT-GLITCH.md](EFFECT-GLITCH.md)

**Tasks:**
- [ ] Slice displacement
- [ ] RGB split / chromatic aberration
- [ ] Block corruption
- [ ] Scanline effects
- [ ] Trigger modes (constant, burst, motion reactive)
- [ ] Application targets (foreground/background/trails/full)

**Success criteria:** Configurable glitch that can target different parts of the frame.

**Estimated effort:** 1-2 sessions

---

### Milestone 7: Effect — Cyberspace

**Objective:** 80s vector graphics background (Max Headroom style).

**Spec:** [EFFECT-CYBERSPACE.md](EFFECT-CYBERSPACE.md)

**Tasks:**
- [ ] 3D projection maths
- [ ] Perspective grid with scrolling
- [ ] Wireframe shapes with rotation
- [ ] Neon colour palettes
- [ ] Glow/bloom post effect
- [ ] Scanline overlay

**Success criteria:** You're composited over a scrolling wireframe grid with floating cubes.

**Estimated effort:** 2 sessions

---

### Milestone 8: Effect — Face Mesh

**Objective:** Wireframe face rendering — the "textures didn't load" look.

**Spec:** [EFFECT-FACEMESH.md](EFFECT-FACEMESH.md)

**Tasks:**
- [ ] Integrate MediaPipe face mesh model (ONNX)
- [ ] Load predefined triangulation topology
- [ ] Basic wireframe rendering (edges only)
- [ ] Solid fill mode (untextured look)
- [ ] Low-poly variation
- [ ] Glow effect on edges
- [ ] Glitched mesh variation

**Success criteria:** Your face rendered as a wireframe mesh that tracks your movements.

**Estimated effort:** 2 sessions

---

### Milestone 9: Polish & Performance

**Objective:** Make it comfortable for daily use.

**Tasks:**
- [ ] Add TensorRT optimisation (first-run compilation, cached engine)
- [ ] Profile and eliminate any frame drops
- [ ] Effect stacking system (combine any background + foreground + post effects)
- [ ] Add FPS counter / performance stats (optional overlay or log)
- [ ] Clean up CLI interface
- [ ] Write basic README with setup instructions
- [ ] Test on actual work calls

**Success criteria:** Runs reliably at 30fps with no visible lag or glitches during a real meeting. Effects can be combined freely.

**Estimated effort:** 1-2 sessions

---

## Future Ideas (Post-MVP)

Things we might add later:

- **More effects:** Edge glow, colour grading, pixelate, mirror
- **Auto-framing:** Face tracking to keep you centred ("Center Stage" style)
- **GUI:** Simple UI to switch effects without CLI
- **Hot-reload:** Change background/effect without restarting
- **Audio reactivity:** Effects that respond to microphone input
- **Depth effects:** Fake bokeh using depth estimation models
- **Eye contact correction:** Gaze redirection using ML

## Technical Risks

| Risk | Mitigation |
|------|------------|
| Model too slow on GPU | Try smaller model variant, or TensorRT optimisation |
| v4l2loopback pixel format issues | Test early in M1, document working formats |
| Alpha edges look bad | Experiment with matte refinement (blur, threshold tweaks) |
| ONNX runtime setup painful | Document exact setup steps, pin versions |

## Dependencies to Install

Before starting:

```bash
# v4l2loopback kernel module
sudo apt install v4l2loopback-dkms v4l2loopback-utils

# Load the module
sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="WebcamFX" exclusive_caps=1

# CUDA (assuming NVIDIA drivers already installed)
# ONNX Runtime will pull CUDA libs, but system CUDA toolkit helps for TensorRT

# Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Getting Started

1. Read through MODELS.md to understand segmentation options
2. Read through STYLE.md for project conventions
3. Start with Milestone 1 — get the basic pipeline working
4. Proceed through milestones in order
