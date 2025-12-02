# Effect: Plasma

Classic demoscene procedural background — animated colour patterns using sine waves.

## The Look

Swirling, pulsing colour gradients that flow and morph continuously. No hard edges, just smooth organic movement. The iconic effect from 90s demos, screensavers, and music visualisers.

## How It Works

For each pixel, compute colour based on position and time:

```rust
fn plasma(x: f32, y: f32, time: f32) -> f32 {
    let v1 = (x * scale + time).sin();
    let v2 = (y * scale + time * 0.7).sin();
    let v3 = ((x + y) * scale * 0.5 + time * 1.3).sin();
    let v4 = ((x * x + y * y).sqrt() * scale + time).sin();
    
    (v1 + v2 + v3 + v4) / 4.0  // Returns -1 to 1
}
```

Map the result through a colour palette to get the final pixel colour.

## Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `speed` | Animation speed multiplier | 1.0 |
| `scale` | Pattern scale (smaller = tighter waves) | 0.02 |
| `complexity` | Number of sine layers | 4 |
| `palette` | Colour palette name | "classic" |

## Palettes

### Classic
The original — cyan, magenta, yellow cycling. Smooth rainbow flow.

### Fire
Black → deep red → orange → yellow → white. Flames.

### Ocean  
Deep blue → cyan → turquoise → white foam.

### Neon
Black → magenta → cyan → white. High contrast, 80s feel.

### Monochrome
Single hue, varying brightness. Subtle, less distracting.

### Custom
User-defined gradient — list of colour stops.

## Variations

### Turbulent Plasma
Add noise displacement to coordinates before computing. Creates more chaotic, roiling effect.

### Directional Flow
Bias the animation so colour appears to flow in a direction — left to right, bottom to top, or radial from centre.

### Reactive Plasma
Tie parameters to external input:
- **Audio** — Speed/scale responds to microphone level
- **Motion** — Your movement affects the plasma intensity

## Implementation Notes

```rust
pub struct PlasmaEffect {
    time: f32,
    speed: f32,
    scale: f32,
    palette: Palette,
}

impl PlasmaEffect {
    fn sample(&self, x: u32, y: u32, width: u32, height: u32) -> Rgb {
        let nx = x as f32 / width as f32;
        let ny = y as f32 / height as f32;
        
        let v = self.plasma(nx, ny);
        self.palette.sample((v + 1.0) / 2.0)  // Map -1..1 to 0..1
    }
}
```

### Performance

Plasma is computed per-pixel, so it's O(width × height) per frame. For 720p that's ~900K pixels.

Options if too slow:
- Compute at lower resolution, upscale
- Use GPU shader (if we add that capability)
- Precompute lookup tables for sin()

Probably fine on CPU at 720p30, but worth profiling.

## Milestone Tasks

- [ ] Basic plasma algorithm (multi-layer sine)
- [ ] Palette system with predefined palettes
- [ ] Custom palette support (gradient definition)
- [ ] Speed and scale parameters
- [ ] Turbulence variation
- [ ] CLI flags for palette selection and parameters
- [ ] (Optional) GPU shader version for performance
