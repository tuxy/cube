use raylib::prelude::*;
use rand::Rng;

mod init;
mod render;
mod physics;
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

struct World { // How does the world interact with objects with properties such as drag and gravity?
    gravity: Vector3,
    // drag: f32, // Far too complex for the scope of this project. Very basic simulation is done
}

#[derive(Clone)]
struct ObjectProperty {
    shape: ShapeKind,
    color: Color,
    stationary: bool,
}

#[derive(Clone)]
struct Object { // Properties of an object (Right now sphere only)
    property: ObjectProperty,
    position: Vector3,
    velocity: Vector3,
    force: Vector3,
    mass: f32,
}
    
fn main() {
    // Initialise program
    let (mut handle, thread, camera) = 
        init::initialisation(init::WINDOW_WIDTH, init::WINDOW_HEIGHT);

    init::initialise_window_options(&mut handle);

    let mut objects:Vec<Object> = Vec::new();
    
    main_loop(handle, thread, camera, &mut objects); // Main program loop
}

fn main_loop(handle: RaylibHandle, thread: RaylibThread, camera: Camera3D, objects: &mut Vec<Object>) {

    let world = World {
        gravity: Vector3 { x: 0.0, y: -9.8, z: 0.0 },
    };

    let mut rng = rand::thread_rng();

    for _ in 0..400 {

        let sphere = Object {
            property: ObjectProperty {
                shape: ShapeKind::Sphere(10.0),
                color: Color::RED,
                stationary: false,
            },
            position: Vector3::new(0.0, 20.0, 0.0),
            velocity: Vector3::new(rng.gen_range(5.0..10.0), 2.0, rng.gen_range(5.0..10.0)),
            force: Vector3::zero(),
            mass: 1.0,
        };

        objects.push(sphere.clone());
    }

    let plane = Object {
        property: ObjectProperty {
            shape: ShapeKind::Plane(250.0, 250.0),
            color: Color::BEIGE,
            stationary: true,
        },
        position: Vector3::zero(),
        velocity: Vector3::zero(),
        force: Vector3::zero(),
        mass: 0.0, // For static objects
    };

    objects.push(plane);

    render::update(handle, thread, camera, objects, &world);
}