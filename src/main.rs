use raylib::prelude::*;
use rand::Rng;

mod init;
mod render;
mod physics;

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
struct Object { // Properties of an object (Right now sphere only)
    radius: f32,
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
            radius: 10.0,
            position: Vector3::zero(),
            velocity: Vector3::new(rng.gen_range(5.0..10.0), 2.0, rng.gen_range(5.0..10.0)),
            force: Vector3::zero(),
            mass: 1.0,
        };

        objects.push(sphere.clone());
    }

    render::update(handle, thread, camera, objects, &world);
}