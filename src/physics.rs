use crate::ShapeKind::*;
use raylib::math::Vector3;

use crate::{Object, World};

// Perform all physics calculations for all objects in each step (TODO: How about ignoring stationary ones?)
pub fn step(object: &mut Object, world: &World, dt: f32) {
    if !object.property.stationary {
        // Gravity (TODO Better way?)             16.67 is the frametime for 60fps. handle.get_frame_time() can get the frametime, just too lazy right now
        object.force += world.gravity * object.mass;
        object.velocity += (object.force / object.mass) * dt;

        object.position += object.velocity;
        object.force = Vector3::zero();
    }
}

// Returns
pub fn handle_collision(i: &Object, j: &Object) -> Vector3 {
    // The 2 objects (Assuming they are both non-stationary)
    // Different collision matches
    match (i.property.shape.clone(), j.property.shape.clone()) {
        (Sphere(radius_i), Sphere(radius_j)) => {
            return ball_collision_response(i, j, radius_i, radius_j);
        }
        (Plane(_, _), Sphere(radius)) => {
            return ball_plane_collision_response(i, j, radius);
        }
        (Sphere(radius), Plane(_, _)) => {
            return ball_plane_collision_response(j, i, radius);
        }
        (_, _) => {
            return i.velocity;
        }
    }
}

fn ball_collision_response(i: &Object, j: &Object, radius_i: f32, radius_j: f32) -> Vector3 {
    // Sphere vs sphere interaction
    let mut vec_position_difference = i.position - j.position;
    let rel_distance = radius_i + radius_j - i.position.distance_to(j.position);
    if i.position.distance_to(j.position) > radius_i + radius_j {
        return i.velocity;
    }

    vec_position_difference.normalize();

                                                             //  0.1 is elasticity
    let additional_v = vec_position_difference * rel_distance * 0.1 * 10.0;
    // These calculations and formulas are very much taken from https://stackoverflow.com/questions/29825110/sphere-to-sphere-collision-c
    // ANd my own calculations from testing & trial/error

    i.velocity + additional_v

    // TODO (I'm going insane)
    // Calculations done, just need to return them
}

fn ball_plane_collision_response(plane: &Object, sphere: &Object, radius: f32) -> Vector3 {
     // Very simple and performant code, BUT ONLY WORKS FOR FLAT PLANES
    let normal = plane.position.normalized().to_array()[1];

    let sphere_position = sphere.position.to_array();
    let distance_to_normal = sphere_position[1] - normal;

    if distance_to_normal > radius {
        return sphere.velocity;
    }

    let sphere_v_array = sphere.velocity.to_array();
        
    let x = sphere_v_array[0] * ((plane.bounciness + sphere.bounciness)/2.0);
    let y = sphere_v_array[1] * ((plane.bounciness + sphere.bounciness)/2.0);
    let z = sphere_v_array[2] * ((plane.bounciness + sphere.bounciness)/2.0);

    // Inverts the y axis only (Bounce), in addition to adding the average bounciness.
    return Vector3::new(x, y.abs(), z); // As a test (No real values)
}