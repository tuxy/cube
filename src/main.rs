use raylib::prelude::*;

const WINDOW_WIDTH:i32 = 800;
const WINDOW_HEIGHT:i32 = 450;

struct World {
    gravity: f32,
    drag: f32,
}

struct Object {
    mass: f32,
    width: f32,
    length: f32,
    velocity: f32,
    displacement: f32,
}

impl Object {
    fn get_terminal_velocity(&self, gravity: f32, drag: f32) -> f32 {
        let mass = self.mass;
        let area = self.width * self.length; 
            // TODO: What happens if an object with non-symmetrical dimensions gets rotated?
        let calc = (2.0*mass*gravity)/(mass*area*drag); // Substitute density for mass for now
        calc.sqrt()
    }
}

fn main() {
    // Initialise program
    let (mut handle, thread, camera) = 
        initialisation(WINDOW_WIDTH, WINDOW_HEIGHT);

    initialise_window_options(&mut handle);
    
    main_loop(handle, thread, camera); // Main program loop
}

fn main_loop(mut handle: RaylibHandle, thread: RaylibThread, mut camera: Camera3D) {

    let world = World {
        gravity: -9.8,
        drag: 0.47, // Real world values,
    };

    let mut cube = Object {
        mass: 1.0,
        width: 20.0,
        length: 20.0,
        velocity: 0.0,
        displacement: 0.0, // Start off stationary
    };

    let max_v = cube.get_terminal_velocity(world.gravity, world.drag);
    println!("{max_v}");

    let mut cube_pos = Vector3::new(0.0, 1.0, 0.0); 
    let plane_pos = Vector3::new(0.0, 0.0, 0.0); // Code that won't be executed every frame

    let mut release_key_pressed = false;

    while !handle.window_should_close() {

        if release_key_pressed { // When release key is = true
            cube_pos = Vector3::new(0.0, 1.0 + cube.displacement/(1.0/handle.get_frame_time()), 0.0); // TODO: Declare object initial position

            // if cube.velocity >= max_v {
            //     cube.acceleration = 0.0;
            // }

            cube.displacement += cube.velocity;
            cube.velocity += world.gravity; // Assuming that up is positive for velocity
        }

        println!("{}", cube.velocity);

        if handle.is_key_pressed(raylib_sys::KeyboardKey::KEY_Z) {
            release_key_pressed = true;
        }

        handle.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        let mut display = handle.begin_drawing(&thread);

        display.clear_background(Color::RAYWHITE);
        {
            let mut d2 = display.begin_mode3D(camera);
            
            d2.draw_cube(cube_pos, 20.0, 20.0, 20.0, Color::RED);
            d2.draw_cube_wires(cube_pos, 20.0, 20.0, 20.0, Color::MAROON);
            // d2.draw_plane(plane_pos, Vector2::new(100.0, 100.0), Color::WHEAT);
        }

        display.draw_fps(0, 0);
    }
}

fn initialisation(w: i32, h: i32) -> (RaylibHandle, RaylibThread, Camera3D) {

    let (handle, thread) = raylib::init()
        .size(w, h)
        .build();

    // Configure camera options
    let camera = Camera3D::perspective(
        Vector3::new(200.0, 50.0, 200.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
    );

    (handle, thread, camera)
}

fn initialise_window_options(handle: &mut RaylibHandle) {
    handle.disable_cursor();
    handle.set_target_fps(60);
}