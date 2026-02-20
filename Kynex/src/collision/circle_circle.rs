// src/collision/circle_circle.rs
use crate::body::Body;
use crate::collision::{Contact, Manifold};
use crate::math::Vec2;
use crate::shape::Shape;

pub fn circle_circle(a_id: usize, b_id: usize, a: &Body, b: &Body) -> Option<Manifold> {
    let (ra, rb) = match (a.shape, b.shape) {
        (Shape::Circle { r: ra }, Shape::Circle { r: rb }) => (ra, rb),
        _ => return None,
    };

    let ab = b.p - a.p;
    let dist2 = ab.len2();
    let r = ra + rb;

    if dist2 >= r * r {
        return None;
    }

    let dist = dist2.sqrt();
    // if centers are on top of each other, pick any normal.
    let normal = if dist > 0.00001 { ab / dist } else { Vec2::X };

    let penetration = r - dist;

    // Contact point: along the normal on A's surface (simple + fine for circles)
    let point = a.p + normal * ra;

    Some(Manifold {
        a: a_id,
        b: b_id,
        contact: Contact { point, normal, penetration },
    })
}
