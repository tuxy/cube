use raylib::math::Vector3;
use crate::ShapeKind::*;

use crate::{Object, World};

// Perform all physics calculations for all objects in each step (TODO: How about ignoring stationary ones?)
pub fn step(object: &mut Object, world: &World, dt: f32) {

    if !object.property.stationary {
        // Gravity (TODO Better way?)             16.67 is the frametime for 60fps. handle.get_frame_time() can get the frametime, just too lazy right now
        object.force += world.gravity * object.mass;
        object.velocity += (object.force/object.mass) * dt;

        object.position += object.velocity;
        object.force = Vector3::zero();
    }
}

// Returns 
pub fn collision_response(i: &Object, j: &Object) { // The 2 objects (Assuming they are both non-stationary)
    // Different collision matches
    match (i.property.shape, j.property.shape) {
        (Sphere(radius_i), Sphere(radius_j)) => { // Sphere vs sphere interaction
            let mut vec_position_difference = i.position - j.position;
            if i.position.distance_to(j.position) >= radius_i + radius_j {
                return;
            }

            vec_position_difference.normalize();

            let vec_velocity_difference = i.velocity - j.velocity;
            let falling_speed = vec_velocity_difference.dot(vec_position_difference);
            // These calculations and formulas are very much taken from https://stackoverflow.com/questions/29825110/sphere-to-sphere-collision-c

            if falling_speed >= 0.0 {
                return;
            }

            // Collision case
            let i_speed = (2.0 * i.mass * falling_speed) / (i.mass + j.mass);
            let j_speed = (falling_speed * (i.mass - j.mass)) / (i.mass + j.mass);
            
            let v1 = i.velocity + (vec_position_difference * i_speed);
            let v2 = j.velocity + (vec_position_difference * (j_speed - falling_speed));

            object1.velocity = v1;
            object2.velocity = v2;

            // TODO (I'm going insane)
            // Calculations done, just need to return them
            
        },
        (_, _) => {
            println!("Collision currently not supported");
            return;
        },
    }
}