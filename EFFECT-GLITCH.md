# Effect: Glitch

Digital corruption, VHS artifacts, datamosh aesthetics. Break reality.

## The Look

Visual corruption that looks like broken video — slices of the image displaced, colours split apart, blocks of noise, scanline interference. Can be subtle (occasional flicker) or heavy (complete breakdown).

## Techniques

### Slice Displacement
Divide frame into horizontal slices, offset some randomly left or right.

```rust
for y in 0..height {
    let slice_id = y / slice_height;
    if should_glitch(slice_id) {
        let offset = random_offset();
        // Copy row with horizontal offset
    }
}
```

### RGB Split (Chromatic Aberration)
Offset colour channels from each other — that "broken LCD" look.

```rust
output.r = sample(x + offset, y);
output.g = sample(x, y);
output.b = sample(x - offset, y);
```

Can also do vertical split, or diagonal.

### Block Corruption
Random rectangular regions get corrupted:
- **Noise fill** — Random static
- **Shift** — Copy from wrong location
- **Repeat** — Copy from previous frame (temporal glitch)
- **Colour invert** — Negative of the block
- **Posterise** — Reduce colour depth harshly

### Scanline Effects

**Interlace** — Drop odd or even scanlines (old TV look)

**Rolling bar** — Dark horizontal band scrolls down the frame (VHS tracking)

**Jitter** — Individual scanlines offset by small random amounts

**Dropout** — Random lines go black or white

### Flip Glitch
Random regions mirror horizontally or vertically. Uncanny, broken display feel.

### Noise Injection
Blend random noise into the image — static, grain, interference patterns.

### Temporal Glitches
Effects that involve time:
- **Frame repeat** — Occasionally show previous frame instead of current
- **Frame blend** — Ghost of previous frame overlaid
- **Timecode jump** — Skip forward/back in frame history randomly

## Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `intensity` | Overall glitch amount (0 = none, 1 = heavy) | 0.3 |
| `slice_probability` | Chance per slice to displace | 0.1 |
| `slice_max_offset` | Maximum displacement in pixels | 30 |
| `rgb_split` | Chromatic aberration offset in pixels | 3 |
| `block_probability` | Chance of block corruption per frame | 0.05 |
| `block_size_range` | Min/max block dimensions | (20, 100) |
| `noise_blend` | Amount of static to mix in (0-1) | 0.0 |
| `scanline_mode` | none, interlace, rolling, jitter | none |

## Trigger Modes

### Constant
Glitch parameters stay steady. Always-on corruption at consistent level.

### Burst
Mostly clean, occasional intense glitch burst, then back to clean. More dramatic.

```rust
if random() < burst_probability {
    apply_heavy_glitch();
    burst_frames_remaining = burst_duration;
}
```

### Motion Reactive
Glitch intensity tied to motion detection. You move → glitch spikes. Still → clean.

Requires tracking frame-to-frame difference.

### Random Walk
Parameters drift over time — intensity waxes and wanes organically.

### Beat Sync (Future)
If we add audio input, sync glitch bursts to music beats.

## Application Targets

With segmentation, we can target where glitches hit:

| Target | Effect |
|--------|--------|
| **Foreground only** | You glitch, background stable |
| **Background only** | You're solid, world corrupts around you |
| **Trails only** | Current frame clean, ghosts corrupted |
| **Full frame** | Everything breaks together |

## Implementation Notes

```rust
pub struct GlitchEffect {
    intensity: f32,
    rgb_split: i32,
    slice_probability: f32,
    block_probability: f32,
    scanline_mode: ScanlineMode,
    trigger: GlitchTrigger,
    rng: StdRng,
}

pub enum GlitchTrigger {
    Constant,
    Burst { probability: f32, duration: u32 },
    MotionReactive { threshold: f32 },
    RandomWalk { speed: f32 },
}

pub enum GlitchTarget {
    Foreground,
    Background,
    Trails,
    FullFrame,
}
```

### Determinism
For consistent glitches (not flickering randomly every frame), seed the RNG per-frame or use coherent noise based on position.

Or embrace the chaos — random every frame for that unstable VHS feel.

## Milestone Tasks

- [ ] Slice displacement effect
- [ ] RGB split / chromatic aberration
- [ ] Block corruption (noise, shift, repeat)
- [ ] Scanline effects (interlace, rolling bar, jitter)
- [ ] Noise injection / static overlay
- [ ] Trigger modes (constant, burst, motion reactive)
- [ ] Application targets (foreground/background/trails/full)
- [ ] CLI configuration for all parameters
- [ ] Presets (subtle, moderate, heavy, VHS, digital)
