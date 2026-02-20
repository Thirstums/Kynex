// src/body.rs
use crate::math::Vec2;
use crate::shape::Shape;

pub type Bodyid = usize;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub restitution: f32, // bounce (0..1)
    pub friction: f32,    // coulomb-ish (0..1-ish)
}

impl Default for Material {
    fn default() -> Self {
        Self { restitution: 0.2, friction: 0.6 }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Body {
    // pose
    pub p: Vec2,   // position
    pub a: f32,    // rotation angle (radians) - we won't use much for circles yet

    // velocities
    pub v: Vec2,   // linear velocity
    pub w: f32,    // angular velocity

    // accumulators
    pub f: Vec2,   // force accumulator
    pub t: f32,    // torque accumulator

    // mass properties
    pub m: f32,
    pub inv_m: f32,
    pub i: f32,
    pub inv_i: f32,

    pub material: Material,
    pub shape: Shape,

    pub is_static: bool,
}

impl Body {
    pub fn new_dynamic(shape: Shape, mass: f32, position: Vec2) -> Self {
        let (m, inv_m) = if mass > 0.0 { (mass, 1.0 / mass) } else { (0.0, 0.0) };

        // inertia for v1:
        // - circle: i = 1/2 m r^2
        // - box:    i = 1/12 m (w^2 + h^2) where w=2hx, h=2hy
        let (i, inv_i) = match shape {
            Shape::Circle { r } => {
                let i = 0.5 * m * r * r;
                (i, if i > 0.0 { 1.0 / i } else { 0.0 })
            }
            Shape::Box { hx, hy } => {
                let w = 2.0 * hx;
                let h = 2.0 * hy;
                let i = (1.0 / 12.0) * m * (w * w + h * h);
                (i, if i > 0.0 { 1.0 / i } else { 0.0 })
            }
        };

        Self {
            p: position,
            a: 0.0,
            v: Vec2::ZERO,
            w: 0.0,
            f: Vec2::ZERO,
            t: 0.0,
            m,
            inv_m,
            i,
            inv_i,
            material: Material::default(),
            shape,
            is_static: false,
        }
    }

    pub fn new_static(shape: Shape, position: Vec2) -> Self {
        Self {
            p: position,
            a: 0.0,
            v: Vec2::ZERO,
            w: 0.0,
            f: Vec2::ZERO,
            t: 0.0,
            m: 0.0,
            inv_m: 0.0,
            i: 0.0,
            inv_i: 0.0,
            material: Material::default(),
            shape,
            is_static: true,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        // Accumulate forces. Cleared each step.
        self.f += force;
    }
}
