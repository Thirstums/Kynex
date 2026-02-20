// src/math/mod.rs

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign,
    Neg, Sub, SubAssign,
};

/// ------------------------------------------------------------
/// Vec2
/// ------------------------------------------------------------
/// This is the fundamental building block of the physics engine.
/// Everything is a vector:
/// - position
/// - velocity
/// - force
/// - normals
/// - offsets from center of mass
///
/// if Vec2 is wrong, your engine is wrong.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Dot product:
    ///
    /// Measures how aligned two vectors are.
    ///
    /// Used heavily in:
    /// - projection
    /// - collision response
    /// - determining velocity along a normal
    ///
    /// if dot(normal, velocity) < 0 → objects moving toward each other.
    #[inline]
    pub fn dot(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// 2D cross product.
    ///
    /// in 3D, cross gives a vector.
    /// in 2D, cross gives a scalar.
    ///
    /// Physically:
    /// Measures "twisting" between two vectors.
    ///
    /// Used in:
    /// - torque
    /// - angular impulse
    /// - SAT calculations
    #[inline]
    pub fn cross(self, rhs: Vec2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Squared length (avoids sqrt for performance).
    ///
    /// Useful for comparisons.
    #[inline]
    pub fn len2(self) -> f32 {
        self.dot(self)
    }

    /// Vector length.
    #[inline]
    pub fn len(self) -> f32 {
        self.len2().sqrt()
    }

    /// Returns a unit-length version of the vector.
    ///
    /// important: collision normals must be normalized.
    /// if they aren't, impulse math breaks.
    #[inline]
    pub fn normalized(self) -> Vec2 {
        let l = self.len();
        if l > 0.0 {
            self / l
        } else {
            Vec2::ZERO
        }
    }

    /// Returns a perpendicular vector (rotated 90° CCW).
    ///
    /// Used in:
    /// - friction direction generation
    /// - SAT axes
    #[inline]
    pub fn perp(self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }

    /// Clamps vector magnitude.
    ///
    /// Prevents runaway velocities.
    #[inline]
    pub fn clamp_len(self, max_len: f32) -> Vec2 {
        let l2 = self.len2();
        if l2 > max_len * max_len {
            self * (max_len / l2.sqrt())
        } else {
            self
        }
    }

    /// Safety check for NaN / infinity.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

/// ------------------------------------------------------------
/// Vec2 Operator Overloads
/// ------------------------------------------------------------
/// These allow natural math syntax like:
/// v1 + v2
/// v * 3.0
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, r: Vec2) -> Vec2 {
        Vec2::new(self.x + r.x, self.y + r.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, r: Vec2) -> Vec2 {
        Vec2::new(self.x - r.x, self.y - r.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, s: f32) -> Vec2 {
        Vec2::new(self.x / s, self.y / s)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, r: Vec2) {
        self.x += r.x;
        self.y += r.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, r: Vec2) {
        self.x -= r.x;
        self.y -= r.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
    }
}

/// ------------------------------------------------------------
/// Cross helpers (CRiTiCAL for rigid body physics)
/// ------------------------------------------------------------
/// in rigid body physics:
///
/// velocity_at_point = v + w × r
///
/// where:
/// - v = linear velocity (Vec2)
/// - w = angular velocity (scalar in 2D)
/// - r = offset from center to contact (Vec2)
///
/// Because 2D angular velocity is scalar,
/// we define custom cross helpers.

/// scalar × vector
///
/// Simulates w × r
#[inline]
pub fn cross_sv(s: f32, v: Vec2) -> Vec2 {
    Vec2::new(-s * v.y, s * v.x)
}

/// vector × scalar
///
/// Simulates r × w
#[inline]
pub fn cross_vs(v: Vec2, s: f32) -> Vec2 {
    Vec2::new(s * v.y, -s * v.x)
}

/// ------------------------------------------------------------
/// Rot
/// ------------------------------------------------------------
/// Stores rotation using sin + cos instead of angle.
/// This avoids recomputing sin/cos every frame.
///
/// Physics engines rotate constantly.
/// This is faster than storing just angle.
#[derive(Debug, Copy, Clone)]
pub struct Rot {
    pub s: f32,
    pub c: f32,
}

impl Rot {
    /// Creates rotation from angle in radians.
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self { s, c }
    }

    /// Rotates a vector.
    ///
    /// Equivalent to multiplying by:
    /// [ cos -sin ]
    /// [ sin  cos ]
    #[inline]
    pub fn rotate(self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.c * v.x - self.s * v.y,
            self.s * v.x + self.c * v.y,
        )
    }

    /// Rotates by inverse rotation (-angle).
    #[inline]
    pub fn inv_rotate(self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.c * v.x + self.s * v.y,
            -self.s * v.x + self.c * v.y,
        )
    }
}

/// ------------------------------------------------------------
/// Transform
/// ------------------------------------------------------------
/// Represents full rigid body transform:
/// - position (translation)
/// - rotation
///
/// Used to move shape vertices from local → world space.
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub p: Vec2, // position
    pub q: Rot,  // rotation
}

impl Transform {
    #[inline]
    pub fn new(p: Vec2, angle: f32) -> Self {
        Self {
            p,
            q: Rot::from_angle(angle),
        }
    }

    /// Applies transform:
    /// world_point = rotate(local_point) + position
    #[inline]
    pub fn apply(self, v: Vec2) -> Vec2 {
        self.q.rotate(v) + self.p
    }

    /// Applies inverse transform.
    ///
    /// Used when converting world → local space.
    #[inline]
    pub fn apply_inv(self, v: Vec2) -> Vec2 {
        self.q.inv_rotate(v - self.p)
    }
}
