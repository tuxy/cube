use raylib::math::Vector3;

use crate::{Object, World};

// Perform all physics calculations for all objects in each step (TODO: How about ignoring stationary ones?)
pub fn step(object: &mut Object, world: &World) {

    if !object.property.stationary {
        // Gravity (TODO Better way?)             16.67 is the frametime for 60fps. handle.get_frame_time() can get the frametime, just too lazy right now
        object.force += world.gravity * object.mass;
        object.velocity += (object.force/object.mass) * 0.01667;

        object.position += object.velocity;
        object.force = Vector3::zero();
    }
}