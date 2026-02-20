// src/shape.rs
use crate::math::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Circle { r: f32 },
    Box { hx: f32, hy: f32 }, // half extents
}

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn overlaps(self, other: Aabb) -> bool {
        self.min.x <= other.max.x &&
        self.max.x >= other.min.x &&
        self.min.y <= other.max.y &&
        self.max.y >= other.min.y
    }
}
