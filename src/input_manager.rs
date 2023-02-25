use notan::prelude::{App, KeyCode};

use crate::{State, camera::Camera2D};

pub fn manage_shortcuts(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.editor_info.paused = !state.editor_info.paused;
    } else if app.keyboard.was_pressed(KeyCode::F) {
        state.editor_info.editor_enabled = !state.editor_info.editor_enabled;
    } else if app.keyboard.is_down(KeyCode::T) {
        state.new_body.dir += 1.;
    } else if app.keyboard.is_down(KeyCode::R) {
        state.new_body.dir -= 1.;
    }
}

pub fn camera_control(app: &mut App, camera: &mut Camera2D, camera_zoom: &mut f32) {
    let mut speed = 5.;
    if app.keyboard.shift() {
        speed = 10.;
    }
    if app.keyboard.is_down(KeyCode::D) {
        camera.pos_add_x(speed);
    }
    if app.keyboard.is_down(KeyCode::A) {
        camera.pos_add_x(-speed);
    }
    if app.keyboard.is_down(KeyCode::S) {
        camera.pos_add_y(speed);
    }
    if app.keyboard.is_down(KeyCode::W) {
        camera.pos_add_y(-speed);
    }
    camera.set_zoom(*camera_zoom);

    if app.keyboard.is_down(KeyCode::Q) {
        *camera_zoom -= *camera_zoom * app.timer.delta_f32();
    }
    if app.keyboard.is_down(KeyCode::E) {
        *camera_zoom += *camera_zoom * app.timer.delta_f32();
    }
}