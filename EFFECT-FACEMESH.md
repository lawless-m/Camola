# Effect: Face Mesh (Untextured Model)

Render your face as a wireframe triangle mesh — like a 3D model with the textures stripped off. The "failed to load" aesthetic.

## The Look

Your face becomes a visible polygon mesh:
- Triangles covering face area
- Just the edges drawn (no fill)
- Mesh follows your facial movements in real-time
- Rest of you (body, hair) can remain normal or also affected

Think: Unfinished video game character, 3D modelling software wireframe view, "I am a simulation" vibes.

## How It Works

### 1. Face Landmark Detection

Get the positions of facial feature points. Options:

**MediaPipe Face Mesh** — 468 landmarks covering the entire face in detail. Very dense mesh.

**dlib 68-point model** — Classic 68 landmarks (eyes, nose, mouth, jawline, eyebrows). Sparser but well-defined.

**OpenCV Facemark** — Similar to dlib, various models available.

For this effect, MediaPipe's 468 points gives us a proper dense mesh. dlib's 68 points would need triangulation but gives a chunkier low-poly look.

### 2. Triangulation

Connect the landmarks into triangles. Two approaches:

**Predefined topology** — MediaPipe provides a canonical face mesh topology (which points connect to which). Use this for consistent triangles frame-to-frame.

**Delaunay triangulation** — Compute triangulation from points each frame. Can be jittery if points move slightly.

MediaPipe's predefined mesh is the way to go — it's stable and designed for this.

### 3. Rendering

For each triangle:
- Project the 3D landmark positions to 2D screen coordinates
- Draw the three edges
- Optionally fill with flat colour or leave hollow

## Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `line_colour` | Wireframe edge colour | cyan |
| `line_width` | Edge thickness in pixels | 1 |
| `fill` | Fill triangles or just edges | edges_only |
| `fill_colour` | If filled, what colour | dark grey |
| `density` | full (468pt) or low_poly (subset) | full |
| `face_only` | Only affect face region | true |
| `glow` | Add glow to edges | false |

## Variations

### Full Wireframe
Just the edges, transparent inside. Your actual face shows through the gaps between lines. Subtle, overlaid effect.

### Solid Untextured
Triangles filled with flat colour (grey, or slight variation per triangle for depth). Completely replaces your face texture. Full "missing texture" look.

### Low-Poly
Use fewer triangles — that chunky PS1-era aesthetic. Either subsample the 468 points or use the 68-point model with Delaunay.

### Depth Shaded
Triangles shaded based on their normal direction — gives sense of 3D form even without texture. Like a clay render.

### Glitched Mesh
Occasionally displace random vertices. Mesh tears, stretches, corrupts. Combine with the glitch effect.

### Colour by Region
Different colours for different face regions:
- Eyes: one colour
- Nose: another
- Mouth: another
- Cheeks: another

### Partial Application
Only apply to part of the face — e.g., one side wireframe, other side normal. Or wireframe fades in/out based on some trigger.

## Face Detection Models

### MediaPipe Face Mesh

**Pros:**
- 468 landmarks — very detailed
- Includes predefined triangulation (no Delaunay needed)
- Also gives 3D depth estimates for each point
- Rust bindings available via `mediapipe-rs` or call via ONNX

**Cons:**
- Another model to run (though lightweight)
- Adds latency

### dlib 68-point

**Pros:**
- Well-established, lots of documentation
- Lighter weight than full mesh
- Good for low-poly aesthetic

**Cons:**
- Need to compute triangulation ourselves
- Fewer points = less detail

### Running in Rust

**Option A: MediaPipe via ONNX**
Export MediaPipe's face mesh model to ONNX, run with `ort` alongside segmentation model.

**Option B: `mediapipe-rs` crate**
Direct Rust bindings to MediaPipe (if sufficiently mature).

**Option C: dlib + OpenCV**
Use `dlib` crate or OpenCV's face landmark detection.

Recommend Option A — we're already using ONNX for segmentation, keep everything in one runtime.

## Implementation Notes

```rust
pub struct FaceMeshEffect {
    // Model
    landmark_model: OrtSession,
    
    // Mesh topology (which vertices connect)
    triangles: Vec<[usize; 3]>,  // Predefined from MediaPipe
    
    // Style
    line_colour: Rgb,
    line_width: u32,
    fill_mode: FillMode,
    
    // State
    landmarks: Vec<Point2>,
}

pub enum FillMode {
    EdgesOnly,
    SolidColour(Rgb),
    DepthShaded,
    PerRegionColour(RegionColours),
}

impl FaceMeshEffect {
    fn detect_landmarks(&mut self, frame: &Frame) -> Vec<Point2> {
        // Run face mesh model
        // Returns 468 (x, y) coordinates
    }
    
    fn render_mesh(&self, frame: &mut Frame, landmarks: &[Point2]) {
        for tri in &self.triangles {
            let p0 = landmarks[tri[0]];
            let p1 = landmarks[tri[1]];
            let p2 = landmarks[tri[2]];
            
            match self.fill_mode {
                EdgesOnly => {
                    draw_line(frame, p0, p1, self.line_colour);
                    draw_line(frame, p1, p2, self.line_colour);
                    draw_line(frame, p2, p0, self.line_colour);
                }
                SolidColour(c) => {
                    fill_triangle(frame, p0, p1, p2, c);
                    // Optionally draw edges on top
                }
                // ...
            }
        }
    }
}
```

### Performance

Face mesh detection is lightweight — MediaPipe runs it at 30fps+ on mobile phones. On desktop GPU it'll be negligible compared to segmentation.

### Combining with Segmentation

Two models running:
1. **Segmentation** (RVM) — Gives alpha matte for compositing
2. **Face Mesh** — Gives landmark positions for wireframe

Both can run on GPU, shouldn't be a problem.

## Integration

The face mesh effect would work on the **foreground** (you), after segmentation but before final compositing.

Pipeline:
```
Frame → Segmentation → Face Mesh Detection → Wireframe Render → Composite over background
```

Can combine with other effects:
- Wireframe face + Plasma background
- Wireframe face + Trails (ghostly wireframe echoes)
- Wireframe face + Glitch (mesh vertices displace randomly)

## Milestone Tasks

- [ ] Integrate face landmark model (MediaPipe face mesh via ONNX)
- [ ] Load predefined triangulation topology
- [ ] Basic wireframe rendering (edges only)
- [ ] Solid fill mode
- [ ] Depth shading variation
- [ ] Low-poly mode (vertex subset or 68-point model)
- [ ] Glow effect on edges
- [ ] Glitched mesh variation (vertex displacement)
- [ ] CLI configuration
- [ ] Performance optimisation (run landmark detection efficiently)
