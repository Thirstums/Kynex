use kynex::body::Body;
use kynex::math::Vec2;
use kynex::shape::Shape;
use kynex::world::World;

fn main() {
    let mut world = World::default();

    world.add_body(Body::new_static(Shape::Circle { r: 1000.0 }, Vec2::new(0.0, -1000.0)));
    world.add_body(Body::new_dynamic(Shape::Circle { r: 0.5 }, 1.0, Vec2::new(0.0, 2.0)));
    world.add_body(Body::new_dynamic(Shape::Circle { r: 0.5 }, 1.0, Vec2::new(0.2, 3.1)));

    let dt = 1.0 / 60.0;
    for step in 0..240 {
        world.step(dt);
        let p0 = world.bodies[1].p;
        if step % 30 == 0 {
            println!("t={:.2} p={:?}", step as f32 * dt, p0);
        }
    }
}
