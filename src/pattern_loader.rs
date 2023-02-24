use notan::math::Vec2;

use crate::mover::Mover;

pub fn load_selected_pattern(planets: &mut Vec<Mover>, pattern: i32) {
    match pattern {
        0 => load_pattern0(planets),
        1 => load_pattern1(planets),
        2 => load_pattern2(planets),
        3 => load_pattern3(planets),
        4 => load_pattern4(planets),
        5 => load_pattern5(planets),
        6 => load_pattern6(planets),
        7 => load_pattern7(planets),
        8 => load_pattern8(planets),
        9 => load_pattern9(planets),
        _ => ()
    }
}

pub fn load_pattern0(planets: &mut Vec<Mover>) {
    planets.clear();
    let center = Vec2::new(0.0, 0.0);

    planets.push(Mover::new(center.x, center.y, 30., 0., 0.).apply_forces(false));

    planets.push(Mover::new(center.x - 700., center.y - 100., 0.0001, -1.8, 10.));
}


pub fn load_pattern1(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(0.0 + 100., 0.0 - 100., 3., 0., 2.));
}

pub fn load_pattern2(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(0.0 + 100., 0.0 - 100., 3., 2., 2.));
}

pub fn load_pattern3(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(0.0 + 100., 0.0 + 300., 3., 0.365, 0.));
}

pub fn load_pattern4(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false));

    planets.push(Mover::new(0.0 + 400., 0.0 + 0., 1., 0., 0.4));
}

pub fn load_pattern5(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0 + 0., 0.0 - 150., 3., 2., 0.0));

    planets.push(Mover::new(0.0 + 0., 0.0 + 150., 3., -2., 0.0));
}

pub fn load_pattern6(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0 + 0., 0.0 - 150., 3., 0.5, 0.0));

    planets.push(Mover::new(0.0 + 0., 0.0 + 150., 3., -0.5, 0.0));
}

pub fn load_pattern7(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0 + 0., 0.0 - 150., 3., 1.35, 0.0));

    planets.push(Mover::new(0.0 + 0., 0.0 + 150., 3., -1.35, 0.0));
}

pub fn load_pattern8(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0 + 150., 0.0 + 150., 3., 4., 0.0));

    planets.push(Mover::new(0.0 - 150., 0.0 + 150., 3., 2., 0.0));

    planets.push(Mover::new(0.0, 0.0 - 150., 3., -1., 0.));
}

pub fn load_pattern9(planets: &mut Vec<Mover>) {
    planets.clear();

    planets.push(Mover::new(0.0 + 200., 0.0 - 0., 3., 0., 2.));
    planets.push(Mover::new(0.0 - 200., 0.0 - 0., 3., 0., -2.));
    planets.push(Mover::new(0.0 - 0., 0.0 - 200., 3., 2., 0.));
    planets.push(Mover::new(0.0 - 0., 0.0 + 200., 3., -2., 0.));
}