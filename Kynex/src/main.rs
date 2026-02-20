use kynex::body::Body;
use kynex::math::Vec2;
use kynex::shape::Shape;
use kynex::world::World;

use macroquad::prelude::*;

#[macroquad::main("Kynex Physics")]
async fn main() {
    let mut world = World::default();

    // Ground
    world.add_body(Body::new_static(
        Shape::Box { hx: 20.0, hy: 1.0 },
        Vec2::new(0.0, -1.0),
    ));

    // Falling circles
for i in 0..5 {
    let id = world.add_body(Body::new_dynamic(
        Shape::Circle { r: 0.5 },
        1.0,
        Vec2::new(0.0, 2.0 + i as f32 * 1.1),
    ));

    world.bodies[id].material.restitution = 0.8;
    world.bodies[id].material.friction = 0.3;
}


let ground = world.add_body(Body::new_static(
    Shape::Box { hx: 20.0, hy: 1.0 },
    Vec2::new(0.0, -1.0),
));
world.bodies[ground].material.restitution = 0.8;



    let dt = 1.0 / 60.0;

    loop {
        clear_background(BLACK);

        world.step(dt);

        draw_world(&world);

        next_frame().await;
    }
}

fn draw_world(world: &World) {
    let scale = 50.0; // world units → pixels

    for body in &world.bodies {
        match body.shape {
            Shape::Circle { r } => {
                draw_circle(
                    body.p.x * scale + screen_width() / 2.0,
                    screen_height() - body.p.y * scale,
                    r * scale,
                    WHITE,
                );
            }

            Shape::Box { hx, hy } => {
                let x = body.p.x * scale + screen_width() / 2.0;
                let y = screen_height() - body.p.y * scale;

                draw_rectangle(
                    x - hx * scale,
                    y - hy * scale,
                    hx * 2.0 * scale,
                    hy * 2.0 * scale,
                    WHITE,
                );
            }
        }
    }
}
