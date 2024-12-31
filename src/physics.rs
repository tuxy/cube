use raylib::math::Vector3;

use crate::{Object, World};

pub fn add_velocity(object: &mut Object, world: &World) {
    // Gravity (TODO Better way?)             16.67 is the frametime for 60fps. handle.get_frame_time() can get the frametime, just too lazy right now
    object.force += world.gravity * object.mass;
    object.velocity += (object.force/object.mass) * 0.01667;

    object.position += object.velocity;
    object.force = Vector3::zero();
}