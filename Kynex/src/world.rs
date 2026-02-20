// src/world.rs
use crate::body::{Body, Bodyid};
use crate::collision::Manifold;
use crate::collision::circle_circle::circle_circle;
use crate::math::{cross_sv, Vec2};


pub struct World {
    pub bodies: Vec<Body>,
    pub gravity: Vec2,
    pub iterations: u32, // solver iterations (8–20)
}

impl Default for World {
    fn default() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vec2::new(0.0, -9.81),
            iterations: 12,
        }
    }
}

impl World {
    pub fn add_body(&mut self, body: Body) -> Bodyid {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    pub fn step(&mut self, dt: f32) {
        // 1) integrate forces -> update velocities
        for b in &mut self.bodies {
            if b.is_static || b.inv_m == 0.0 { continue; }

            // semi-implicit Euler:
            b.v += (b.f * b.inv_m + self.gravity) * dt;
            b.w += (b.t * b.inv_i) * dt;

            // clear accumulators for next step
            b.f = Vec2::ZERO;
            b.t = 0.0;
        }

        // 2) Broadphase (naive O(n^2) for v1) + Narrowphase
        let mut contacts: Vec<Manifold> = Vec::new();

        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                // skip static-static
                if self.bodies[i].is_static && self.bodies[j].is_static { continue; }

                // For now: only circle-circle collisions
                if let Some(m) = circle_circle(i, j, &self.bodies[i], &self.bodies[j]) {
                    contacts.push(m);
                }
            }
        }

        // 3) Solve velocity constraints (impulses) iteratively
        for _ in 0..self.iterations {
            for m in &contacts {
                self.solve_contact(*m);
            }
        }

        // 4) integrate positions
        for b in &mut self.bodies {
            if b.is_static { continue; }
            b.p += b.v * dt;
            b.a += b.w * dt;
        }

        // 5) Positional correction (prevents sinking / improves stacking)
        for m in &contacts {
            self.positional_correction(*m);
        }
    }

    fn solve_contact(&mut self, m: Manifold) {
        // (Borrowing trick): temporarily split mutable borrows
        let (a, b) = get2_mut(&mut self.bodies, m.a, m.b);
        let c = m.contact;

        // rA/rB: contact point relative to centers
        let ra = c.point - a.p;
        let rb = c.point - b.p;

        // velocity at contact = v + w × r
        let va = a.v + cross_sv(a.w, ra);
        let vb = b.v + cross_sv(b.w, rb);

        let rv = vb - va;

        // relative velocity along the normal
        let vel_along_normal = rv.dot(c.normal);

        // if they’re separating, don’t apply normal impulse
        if vel_along_normal > 0.0 {
            return;
        }

        // restitution (bounce)
        let e = a.material.restitution.min(b.material.restitution);

        // effective mass along normal:
        // 1 / (inv_mA + inv_mB + rotational terms)
        let ra_cn = ra.cross(c.normal);
        let rb_cn = rb.cross(c.normal);

        let inv_mass_sum =
            a.inv_m + b.inv_m +
            (ra_cn * ra_cn) * a.inv_i +
            (rb_cn * rb_cn) * b.inv_i;

        if inv_mass_sum == 0.0 { return; }

        // impulse scalar
        let j = -(1.0 + e) * vel_along_normal / inv_mass_sum;

        let impulse = c.normal * j;

        // Apply linear impulses
        a.v -= impulse * a.inv_m;
        b.v += impulse * b.inv_m;

        // Apply angular impulses: w += inv_i * (r × impulse)
        a.w -= a.inv_i * ra.cross(impulse);
        b.w += b.inv_i * rb.cross(impulse);

        // ---- friction impulse ----
        // Tangent direction: remove normal component from rv
        let mut tangent = rv - c.normal * rv.dot(c.normal);
        tangent = tangent.normalized();

        let ra_ct = ra.cross(tangent);
        let rb_ct = rb.cross(tangent);

        let inv_mass_t =
            a.inv_m + b.inv_m +
            (ra_ct * ra_ct) * a.inv_i +
            (rb_ct * rb_ct) * b.inv_i;

        if inv_mass_t == 0.0 { return; }

        let jt = -rv.dot(tangent) / inv_mass_t;

        // Coulomb friction clamp
        let mu = (a.material.friction * b.material.friction).sqrt();
        let jt_clamped = jt.clamp(-mu * j, mu * j);

        let friction_impulse = tangent * jt_clamped;

        a.v -= friction_impulse * a.inv_m;
        b.v += friction_impulse * b.inv_m;

        a.w -= a.inv_i * ra.cross(friction_impulse);
        b.w += b.inv_i * rb.cross(friction_impulse);
    }

    fn positional_correction(&mut self, m: Manifold) {
        let (a, b) = get2_mut(&mut self.bodies, m.a, m.b);
        let c = m.contact;

        // tweakables
        let percent = 0.8;   // 0.2–0.8
        let slop = 0.01;     // small allowance

        let correction_mag = ((c.penetration - slop).max(0.0) / (a.inv_m + b.inv_m)) * percent;
        let correction = c.normal * correction_mag;

        if !a.is_static {
            a.p -= correction * a.inv_m;
        }
        if !b.is_static {
            b.p += correction * b.inv_m;
        }
    }
}

/// Helper to get two mutable refs from one Vec without borrow fights.
fn get2_mut<T>(v: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j);
    if i < j {
        let (left, right) = v.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = v.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}
