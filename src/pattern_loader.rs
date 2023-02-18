use notan::{prelude::App, math::Vec2};

use crate::mover::Mover;

pub fn load_selected_pattern(planets: &mut Vec<Mover>, app: &mut App, pattern: i32) {
    match pattern {
        0 => load_pattern0(planets, app),
        1 => load_pattern1(planets, app),
        2 => load_pattern2(planets, app),
        3 => load_pattern3(planets, app),
        4 => load_pattern4(planets, app),
        5 => load_pattern5(planets, app),
        6 => load_pattern6(planets, app),
        7 => load_pattern7(planets, app),
        8 => load_pattern8(planets, app),
        9 => load_pattern9(planets, app),
        _ => ()
    }
}

pub fn load_pattern0(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();
    let center = Vec2::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2.);

    planets.push(Mover::new(center.x, center.y, 30., 0., 0.).apply_forces(false));

    planets.push(Mover::new(center.x - 700., center.y - 100., 0.0001, -1.8, 10.));
}


pub fn load_pattern1(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 100., app.window().height() as f32 / 2. - 100., 3., 0., 2.));
}

pub fn load_pattern2(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 100., app.window().height() as f32 / 2. - 100., 3., 2., 2.));
}

pub fn load_pattern3(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 100., app.window().height() as f32 / 2. + 300., 3., 0.365, 0.));
}

pub fn load_pattern4(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 400., app.window().height() as f32 / 2. + 0., 1., 0., 0.4));
}

pub fn load_pattern5(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. - 150., 3., 2., 0.0));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. + 150., 3., -2., 0.0));
}

pub fn load_pattern6(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. - 150., 3., 0.5, 0.0));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. + 150., 3., -0.5, 0.0));
}

pub fn load_pattern7(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. - 150., 3., 1.35, 0.0));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 0., app.window().height() as f32 / 2. + 150., 3., -1.35, 0.0));
}

pub fn load_pattern8(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2. + 150., app.window().height() as f32 / 2. + 150., 3., 4., 0.0));

    planets.push(Mover::new(app.window().width() as f32 / 2. - 150., app.window().height() as f32 / 2. + 150., 3., 2., 0.0));

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2. - 150., 3., -1., 0.));
}

pub fn load_pattern9(planets: &mut Vec<Mover>, app: &mut App) {
    planets.clear();

    planets.push(Mover::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(app.window().width() as f32 / 2. + 200., app.window().height() as f32 / 2. - 0., 3., 0., 2.));
    planets.push(Mover::new(app.window().width() as f32 / 2. - 200., app.window().height() as f32 / 2. - 0., 3., 0., -2.));
    planets.push(Mover::new(app.window().width() as f32 / 2. - 0., app.window().height() as f32 / 2. - 200., 3., 2., 0.));
    planets.push(Mover::new(app.window().width() as f32 / 2. - 0., app.window().height() as f32 / 2. + 200., 3., -2., 0.));
}