# Webcam FX — Project Documentation

A Rust application for real-time webcam segmentation and background effects, designed for video calls (Teams, Zoom, etc.) on Linux.

## What This Project Does

Captures your webcam feed, segments you from the background using a neural network, then composites you onto replacement backgrounds or effects. Outputs to a v4l2loopback virtual camera that video conferencing apps see as a real webcam.

## Document Index

| Document | Purpose |
|----------|---------|
| [MODELS.md](MODELS.md) | Segmentation models — what's available, tradeoffs, how to run them |
| [EFFECTS.md](EFFECTS.md) | Effects index — links to individual effect specs |
| [STYLE.md](STYLE.md) | Rust coding conventions, project structure, dependencies |
| [PLAN.md](PLAN.md) | Implementation roadmap, milestones |

### Effect Specifications

| Effect | Document |
|--------|----------|
| Trails (ghosting, blur, chroma, glitch) | [EFFECT-TRAILS.md](EFFECT-TRAILS.md) |
| Plasma (demoscene colours) | [EFFECT-PLASMA.md](EFFECT-PLASMA.md) |
| Glitch (corruption, RGB split) | [EFFECT-GLITCH.md](EFFECT-GLITCH.md) |
| Cyberspace (Max Headroom style) | [EFFECT-CYBERSPACE.md](EFFECT-CYBERSPACE.md) |
| Face Mesh (wireframe face) | [EFFECT-FACEMESH.md](EFFECT-FACEMESH.md) |

## Hardware Context

- NVIDIA RTX 3070/4070 (8GB VRAM)
- Linux (Ubuntu assumed)
- Standard USB webcam

## High-Level Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐     ┌──────────────┐
│   Webcam    │────▶│  Segmentation │────▶│   Effects   │────▶│  v4l2loopback │
│  (1080p)    │     │    Model      │     │  Compositor │     │   (720p)      │
└─────────────┘     └──────────────┘     └─────────────┘     └──────────────┘
                          │
                    CUDA/TensorRT
                     (GPU accelerated)
```

## Quick Reference

**Target output:** 720p @ 30fps to virtual camera  
**Capture resolution:** 1080p (gives headroom for any cropping/framing effects)  
**Segmentation:** Alpha matte via trait-based model abstraction (RobustVideoMatting primary, MODNet fallback)  
**Effects:** Background replacement, trails/ghosting, plasma, glitch, cyberspace, face mesh

## Getting Started

See [PLAN.md](PLAN.md) for the implementation order. We'll build in stages:

1. Basic pipeline (camera → v4l2loopback passthrough)
2. Add segmentation model
3. Add background replacement
4. Add procedural effects
