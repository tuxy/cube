use crate::ShapeKind::*;
use raylib::math::Vector3;

use crate::{Object, World};

pub fn smooth(vec: Vector3) -> Vector3 { // Custom vector smoothing

    let vec_array: [f32; 3] = vec.to_array();
    let smooth_factor: f32 = 1000.0; // Lower = smoother

    let smoothed_y = (vec_array[1] * smooth_factor).round().floor() / smooth_factor;

    Vector3::new(vec_array[0], smoothed_y, vec_array[0])
}

// Perform all physics calculations for all objects in each step (TODO: How about ignoring stationary ones?)
pub fn step(object: &Object, world: &World, dt: f32) -> (Vector3, Vector3) { // Returns (Velocity, Pos)
    if !object.property.stationary {

        // Gravity (TODO Better way?)             16.67 is the frametime for 60fps. handle.get_frame_time() can get the frametime, just too lazy right now
        let force = object.force + world.gravity * object.mass;
        let v = object.velocity + (force / object.mass) * dt;

        let pos = object.position + object.velocity;
        
        return (v, pos);
    }
    return (Vector3::zero(), Vector3::zero());
}

// Read-only for objects, only returns what will happen
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
            return Vector3::zero();
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
        return sphere.velocity; // Do nothing
    }

    let sphere_v_array = sphere.velocity.to_array();
        
    let x = sphere_v_array[0] * ((plane.bounciness + sphere.bounciness)/2.0);
    let y = sphere_v_array[1] * ((plane.bounciness + sphere.bounciness)/2.0);
    let z = sphere_v_array[2] * ((plane.bounciness + sphere.bounciness)/2.0);

    // Inverts the y axis only (Bounce), in addition to adding the average bounciness.
    return Vector3::new(x, y.abs(), z); // As a test (No real values)
}
