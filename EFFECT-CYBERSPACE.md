# Effect: Cyberspace (Max Headroom Style)

80s vector graphics background — wireframe grids, neon lines, rotating geometry. The aesthetic of early CGI, music videos, and Max Headroom's iconic backdrop.

## The Look

- Black void background
- Neon-coloured wireframe grid receding to a vanishing point
- Perspective lines creating a corridor or infinite plane
- Floating geometric shapes (cubes, pyramids) rotating slowly
- No filled surfaces — pure edges and lines
- Slight glow on the lines
- Optional CRT scanline overlay

Think: Tron, Max Headroom, 80s music videos, early Amiga/ST demos.

## Components

### The Grid

A flat plane rendered in perspective, scrolling toward or away from camera.

```
        ·─────────·─────────·─────────·
       ╱         ╱         ╱         ╱
      ·─────────·─────────·─────────·
     ╱         ╱         ╱         ╱
    ·─────────·─────────·─────────·
   ╱         ╱         ╱         ╱
  ═══════════════════════════════════
```

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `grid_lines` | Number of lines in each direction | 16 |
| `scroll_speed` | How fast grid moves toward camera | 1.0 |
| `horizon` | Vertical position of vanishing point (0-1) | 0.4 |
| `fov` | Field of view / perspective strength | 60 |

### The Tunnel / Corridor

Grid on floor, ceiling, and walls — flying through a wireframe corridor.

Four grids (floor, ceiling, left wall, right wall) all converging to centre vanishing point.

### Floating Geometry

Wireframe 3D shapes drifting across the scene:

**Shapes:**
- Cube
- Tetrahedron (pyramid)
- Octahedron
- Icosahedron
- Torus (if we're feeling fancy)

**Behaviour:**
- Slow rotation on one or more axes
- Drift across frame (left to right, or toward/away)
- Multiple shapes at different depths
- Shapes can enter and exit frame

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `shape` | Which geometry | cube |
| `rotation_speed` | Degrees per frame | 0.5 |
| `drift_speed` | Movement across frame | 0.3 |
| `count` | Number of shapes | 2 |

### Colour Palette

Classic 80s neon:

| Name | Colours |
|------|---------|
| **Neon** | Cyan (#00FFFF), Magenta (#FF00FF), on black |
| **Tron** | Cyan (#00D4FF), Orange (#FF6600), on black |
| **Matrix** | Green (#00FF00) only, on black |
| **Synthwave** | Pink (#FF1493), Blue (#00BFFF), Purple (#8A2BE2) |
| **Mono** | Single colour (e.g., cyan) at varying brightness |

Grid and shapes can be same colour or different. Colour can slowly cycle.

### Post Effects

**Glow / Bloom**
Lines have soft glow around them — expand bright pixels slightly, blur, blend back.

**Scanlines**
Horizontal lines at regular intervals, subtle darkening. CRT monitor feel.

**Vignette**
Darken edges of frame, focus attention on centre.

**Colour cycling**
Hue rotates slowly over time — the whole palette shifts.

## Rendering Approach

### 3D Projection

Basic perspective projection:

```rust
fn project(point: Vec3, camera_z: f32, fov: f32, screen: (u32, u32)) -> Vec2 {
    let scale = fov / (camera_z + point.z);
    Vec2 {
        x: point.x * scale + screen.0 / 2,
        y: point.y * scale + screen.1 / 2,
    }
}
```

### Line Drawing

Bresenham's algorithm or similar for drawing wireframe edges. Anti-aliased lines look better but cost more.

### Shape Rotation

Standard rotation matrices:

```rust
fn rotate_y(point: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: point.x * angle.cos() + point.z * angle.sin(),
        y: point.y,
        z: -point.x * angle.sin() + point.z * angle.cos(),
    }
}
```

## Variations

### Static Grid
Grid doesn't scroll — just a static backdrop with maybe slow colour shift.

### Starfield
Instead of grid, points (stars) flying toward camera. Classic "warp speed" effect.

### Combination
Grid floor + starfield above horizon + floating shapes.

### Reactive
Grid scroll speed or shape rotation tied to audio level or your motion.

## Implementation Notes

```rust
pub struct CyberspaceEffect {
    // Grid
    grid_enabled: bool,
    grid_lines: u32,
    grid_scroll: f32,
    scroll_speed: f32,
    horizon: f32,
    
    // Shapes
    shapes: Vec<WireframeShape>,
    
    // Style
    palette: CyberPalette,
    glow_enabled: bool,
    scanlines_enabled: bool,
    
    // Animation
    time: f32,
}

pub struct WireframeShape {
    geometry: Geometry,
    position: Vec3,
    rotation: Vec3,
    rotation_speed: Vec3,
    drift: Vec3,
}

pub enum Geometry {
    Cube,
    Tetrahedron,
    Octahedron,
    Icosahedron,
}
```

## Milestone Tasks

- [ ] 3D projection maths (world → screen)
- [ ] Line drawing with anti-aliasing
- [ ] Perspective grid (floor plane)
- [ ] Grid scrolling animation
- [ ] Tunnel mode (floor + ceiling + walls)
- [ ] Wireframe cube with rotation
- [ ] Additional shapes (tetrahedron, octahedron)
- [ ] Multiple floating shapes
- [ ] Colour palettes (neon, tron, synthwave)
- [ ] Glow/bloom post effect
- [ ] Scanline overlay
- [ ] CLI configuration
- [ ] Presets (max_headroom, tron, synthwave)
