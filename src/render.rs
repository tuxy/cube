use raylib::prelude::*;
use crate::{Object, World};
use crate::physics;

pub fn update(mut handle: RaylibHandle, thread: RaylibThread, mut camera: Camera3D, objects: &mut Vec<Object>, world: &World) {
    while !handle.window_should_close() {

        handle.update_camera(&mut camera, CameraMode::CAMERA_FREE); // Can be moved around with wasd

        let mut display = handle.begin_drawing(&thread);

        display.clear_background(Color::RAYWHITE);
        {
            let mut d2 = display.begin_mode3D(camera);
            

            // MAIN CUBE
            for i in &mut *objects {

                physics::add_velocity(i, world);

                d2.draw_sphere(i.position, i.radius, Color::RED);
                // d2.draw_sphere_wires(i.position, i.radius, 10, 10, Color::MAROON);
            }
            
            // reference grid
            d2.draw_grid(50, 5.0);
        }

        println!("{}", display.get_fps());

        display.draw_fps(0, 0);
        // display.draw_text("Press Z to start/stop simulation", init::WINDOW_WIDTH/2 - 50, init::WINDOW_HEIGHT-20, 16, Color::BLACK);
    }
}