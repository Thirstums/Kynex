# Kynex

## Vec2 (Fundamental Unit)

```rust
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

This is literally just a 2D Vector

Position → (x, y)

Velocity → (vx, vy)

Force → (fx, fy)

Normal → direction vector

Offset from center → r

## Dot product

```rust
pub fn dot(self, rhs: Vec2) -> f32
```

formula

```
x1*x2 + y1*y2
```