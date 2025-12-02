# Effects Overview

Each effect has its own document with full specification and milestone tasks.

## Effect Documents

| Effect | Description | Document |
|--------|-------------|----------|
| **Trails** | Ghosting/echo — fading copies follow you | [EFFECT-TRAILS.md](EFFECT-TRAILS.md) |
| **Plasma** | Classic demoscene animated colours | [EFFECT-PLASMA.md](EFFECT-PLASMA.md) |
| **Glitch** | Digital corruption, RGB split, displacement | [EFFECT-GLITCH.md](EFFECT-GLITCH.md) |
| **Cyberspace** | 80s vector graphics, wireframe grids | [EFFECT-CYBERSPACE.md](EFFECT-CYBERSPACE.md) |
| **Face Mesh** | Wireframe face, untextured model look | [EFFECT-FACEMESH.md](EFFECT-FACEMESH.md) |

## Effect Trait

All effects implement a common interface:

```rust
pub trait Effect {
    fn process(
        &mut self,
        frame: &Frame,
        matte: &Matte,
        history: &FrameHistory,
        time: f32,
    ) -> Frame;
    
    fn name(&self) -> &str;
}
```

## Effect Categories

### Foreground Effects
Apply to you (the segmented person):
- Trails (with variations: blur, chroma, glitch)

### Background Effects  
Replace or modify what's behind you:
- Plasma
- Cyberspace
- Static image / video

### Full Frame Effects
Apply to entire output:
- Glitch
- Scanlines
- Colour grading

### Post Effects
Applied after compositing:
- Glow / bloom
- Vignette
- CRT simulation

## Stacking

Effects can be combined:
- Cyberspace background + Trails foreground
- Plasma background + Glitch on trails
- Any background + any foreground effect + any post effect

## Adding New Effects

1. Create `EFFECT-{NAME}.md` with spec and milestone tasks
2. Add entry to this index
3. Implement in `src/effects/{name}.rs`
4. Register in effect factory
