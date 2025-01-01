use raylib::prelude::*;

pub const WINDOW_WIDTH:i32 = 1280;
pub const WINDOW_HEIGHT:i32 = 720;

pub fn initialisation(w: i32, h: i32) -> (RaylibHandle, RaylibThread, Camera3D) {

    let (handle, thread) = raylib::init()
        .size(w, h)
        .msaa_4x()
        .vsync()
        .build();

    // Configure camera options
    let camera = Camera3D::perspective(
        Vector3::new(100.0, 25.0, 100.0),
        Vector3::new(0.0, 10.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        55.0,
    );

    (handle, thread, camera)
}

pub fn initialise_window_options(handle: &mut RaylibHandle) {
    handle.disable_cursor();
    handle.set_target_fps(60);
}