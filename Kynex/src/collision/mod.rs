// src/collision/mod.rs
pub mod circle_circle;

use crate::math::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Contact {
    pub point: Vec2,
    pub normal: Vec2,      // from A -> B
    pub penetration: f32,  // how deep they overlap
}

#[derive(Debug, Copy, Clone)]
pub struct Manifold {
    pub a: usize,
    pub b: usize,
    pub contact: Contact,
}
