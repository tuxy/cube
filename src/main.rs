use rand::Rng;
use raylib::prelude::*;

mod init;
mod physics;
mod render;
mod shapes;

use shapes::ShapeKind;

/*
FEATURES (going to be) SUPPORTED IN SIMULATION:
- Gravity
- Drag
- Terminal velocity (TODO)
FUTURE FEATURES:
- Angular momentum
- VERY BASIC collision (No angular momentum for this. Just simple contact)
*/

struct World {
    // How does the world interact with objects with properties such as drag and gravity?
    gravity: Vector3,
    // drag: f32, // Far too complex for the scope of this project. Very basic simulation is done
}

#[derive(Clone, PartialEq)]
struct ObjectProperty {
    shape: ShapeKind,
    color: Color,
    wire_color: Color,
    stationary: bool,
}

#[derive(Clone, PartialEq)]
struct Object {
    // Properties of an object (Right now sphere only)
    property: ObjectProperty,
    position: Vector3,
    relation: Vec<f32>, // NOT USED YET
    velocity: Vector3,
    force: Vector3,
    bounciness: f32, // OUT OF 1.0
    mass: f32,
}

fn main() {
    // Initialise program
    let (mut handle, thread, camera) =
        init::initialisation(init::WINDOW_WIDTH, init::WINDOW_HEIGHT);

    init::initialise_window_options(&mut handle);

    let mut objects: Vec<Object> = Vec::new();

    main_loop(handle, thread, camera, &mut objects); // Main program loop
}

fn main_loop(
    handle: RaylibHandle,
    thread: RaylibThread,
    camera: Camera3D,
    objects: &mut Vec<Object>,
) {
    let world = World {
        gravity: Vector3 {
            x: 0.0,
            y: -9.8,
            z: 0.0,
        },
    };

    let mut rng = rand::thread_rng();

    for _ in 0..200 { // Spawn 200 spheres, the upper limit for my graphics card.
        let sphere: Object = Object {
            property: ObjectProperty {
                shape: ShapeKind::Sphere(4.0),
                color: Color::RED,
                wire_color: Color::BLACK,
                stationary: false,
            },
            position: Vector3::new(rng.gen_range(0.0..100.0), rng.gen_range(40.0..70.0), rng.gen_range(0.0..100.0)),
            relation: vec![],
            velocity: Vector3::new(0.0, rng.gen_range(0.2..1.0), 0.0),
            bounciness: 1.0,
            force: Vector3::zero(),
            mass: 1.0,
        };
        objects.push(sphere.clone());
    }

    let plane = Object {
        property: ObjectProperty {
            shape: ShapeKind::Plane(2500.0, 2500.0),
            color: Color::ALICEBLUE,
            wire_color: Color::BLUE,
            stationary: true,
        },
        position: Vector3::zero(),
        relation: vec![],
        velocity: Vector3::zero(),
        bounciness: 0.2,
        force: Vector3::zero(),
        mass: 0.0,
    };

    objects.push(plane);

    render::update(handle, thread, camera, objects, &world);
}
