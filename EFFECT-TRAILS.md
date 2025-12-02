# Effect: Trails

Ghosting / echo effect — you move and leave fading copies of yourself behind.

## The Look

You walk across frame, translucent copies of yourself trail behind, fading out over time. Classic demoscene / 80s video mixer aesthetic.

```
You move →  👻 👻 👻 🧍
           10% 20% 40% 100%
```

Because we have segmentation, trails are *just you* — background stays clean.

## How It Works

1. Maintain a ring buffer of recent frames
2. Every N frames, snapshot current foreground (you × alpha) into buffer
3. When compositing:
   - Start with background
   - Layer oldest trail at low opacity
   - Layer each subsequent trail at increasing opacity
   - Current frame on top at 100%

## Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `interval` | Frames between snapshots | 5 |
| `count` | Number of trail frames to keep | 6 |
| `decay` | Opacity falloff curve (linear, exponential) | exponential |
| `fade_start` | Opacity of oldest trail | 0.1 |
| `fade_end` | Opacity of newest trail | 0.5 |

## Variations

### Colour Shift (Chroma Trails)
Each trail tinted a different hue — rainbow ghost effect.

- Oldest trail: red shift
- Middle trails: through the spectrum
- Newest trail: blue shift
- Or: all trails same tint (e.g., cyan ghosts)

**Extra parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `hue_shift` | Degrees to shift per trail | 30 |
| `saturation` | Colour intensity | 0.7 |

### Blur Trails
Older trails get progressively blurred — motion blur / dreamy look.

- Current frame: sharp
- Recent trails: slight blur
- Oldest trails: heavy blur

**Extra parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `blur_start` | Blur radius for newest trail | 1 |
| `blur_end` | Blur radius for oldest trail | 10 |

### Glitch Trails
Digital corruption on the echoes — current frame clean, ghosts are broken.

- **Slice displacement** — Horizontal slices offset randomly per trail
- **RGB split** — Colour channels misaligned
- **Flip** — Random horizontal/vertical flip per trail
- **Block corruption** — Random rectangles replaced with noise
- **Interlace** — Only even/odd scanlines, alternating per trail

**Extra parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `glitch_intensity` | How corrupted (0-1) | 0.3 |
| `rgb_split` | Chromatic aberration pixels | 3 |
| `slice_probability` | Chance per slice to displace | 0.15 |

### Combined
Variations can stack — blurry chromatic glitch trails, etc.

## Implementation Notes

```rust
pub struct TrailsEffect {
    buffer: VecDeque<TrailFrame>,
    interval: u32,
    count: usize,
    frame_counter: u32,
    decay: DecayCurve,
    variation: TrailVariation,
}

pub struct TrailFrame {
    rgba: Vec<u8>,      // Premultiplied foreground
    timestamp: u32,
}

pub enum TrailVariation {
    None,
    Chroma { hue_shift: f32 },
    Blur { start: f32, end: f32 },
    Glitch { intensity: f32, rgb_split: i32 },
    Combined(Vec<TrailVariation>),
}
```

## Milestone Tasks

- [ ] Implement ring buffer for frame history
- [ ] Basic trails compositing with opacity decay
- [ ] Configurable interval and count
- [ ] Chroma variation (hue shift per trail)
- [ ] Blur variation (progressive blur)
- [ ] Glitch variation (slice, RGB split, flip)
- [ ] CLI flags to select variation and parameters
- [ ] Allow combining variations
