use raylib::prelude::*;
use crate::{Object, World};
use crate::physics;
use crate::shapes::ShapeKind::*;

use crate::init;

pub fn update(mut handle: RaylibHandle, thread: RaylibThread, mut camera: Camera3D, objects: &mut Vec<Object>, world: &World) {

    let mut running = false; // Paused by default
    let mut objects = objects.clone();

    while !handle.window_should_close() {

        if handle.is_key_pressed(KeyboardKey::KEY_Z) {
            running = true;
        }

        let dt = handle.get_frame_time();

        handle.update_camera(&mut camera, CameraMode::CAMERA_FREE); // Can be moved around with wasd

        let mut display = handle.begin_drawing(&thread);

        display.clear_background(Color::BLACK);
        {
            let mut d2 = display.begin_mode3D(camera);

            let mut objects_copy = objects.clone();

            if running { // If simulation is running
                for i in &mut objects {
                    for j in &mut objects_copy {
                        if i != j {
                            i.velocity =  physics::handle_collision(i, j);
                        }
                    }
                }
            }
            // This is extremely computationally expensive
            // One way to fix this is to order the priority of checking by relative distance

            // MAIN CUBE 
            for i in &mut *objects {

                if running { // Steps each object in the list by "1"
                    physics::step(i, world, dt);
                }

                match i.property.shape {
                    Sphere(r) => {
                        d2.draw_sphere(i.position, r, i.property.color);
                        d2.draw_sphere_wires(i.position, r, 10, 10, i.property.wire_color);
                    },
                    Plane(x, z) => {
                        d2.draw_plane(i.position, Vector2::new(x, z), i.property.color);
                    },
                }
            }
            
            // reference grid
            d2.draw_grid(500, 5.0);
        }

        // println!("{}", display.get_fps());

        display.draw_fps(0, 0);
        display.draw_text("Press Z to start simulation", init::WINDOW_WIDTH/2 - 50, init::WINDOW_HEIGHT-20, 16, Color::BLACK);
    }
}