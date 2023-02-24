use std::ops::AddAssign;

use notan::{math::Vec2, draw::{Draw, DrawShapes}, prelude::{Color, App}, random::rand::random};

use crate::{vec_math, TRAIL_TEX_WIDTH, G, TRAIL_TEX_HEIGHT};

#[derive(Clone, Copy)]
pub struct Mover {
    pub pos: Vec2,
    pub delta_pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub m: f32,
    pub r: f32,
    pub trail_color: Color,
    pub apply_forces: bool,
    pub selected: bool
}

impl Mover {
    pub fn new(x: f32, y: f32, m: f32, fx: f32, fy: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            delta_pos: Vec2::new(x, y),
            vel: Vec2::new(fx, fy),
            acc: Vec2::ZERO,
            m,
            r: m.sqrt() * 10.,
            trail_color: Color::from_rgb(0.5 + random::<f32>() / 2., 0.5 + random::<f32>() / 2., 0.5 + random::<f32>() / 2.),
            apply_forces: true,
            selected: false
        }
    }

    pub fn update(&mut self, _app: &mut App) {
        if self.apply_forces {
            self.vel.add_assign(self.acc);
            self.pos.add_assign(self.vel);

            vec_math::limit(&mut self.vel, 10.);
            self.acc = Vec2::ZERO;
        }
    }

    pub fn save_delta_pos(&mut self) {
        self.delta_pos = self.pos.clone();
    }

    pub fn render_trail(&self, draw: &mut Draw) {
        draw.line(((TRAIL_TEX_WIDTH as f32 / 2. + self.pos.x), (TRAIL_TEX_HEIGHT as f32 / 2. + self.pos.y)), ((TRAIL_TEX_WIDTH as f32 / 2. + self.delta_pos.x), (TRAIL_TEX_HEIGHT as f32 / 2. + self.delta_pos.y))).width(2.).color(self.trail_color);
    }

    pub fn render(&self, draw: &mut Draw) {
        if !self.selected {
            draw.ellipse((self.pos.x, self.pos.y), (self.r, self.r));
        } else {
            draw.ellipse((self.pos.x, self.pos.y), (self.r, self.r)).color(Color::RED);
        }
    }

    pub fn apply_force(&mut self, force: &Vec2) {
        let f = Vec2::new(force.x / self.m, force.y / self.m);
        self.acc.add_assign(f);
    }

    pub fn apply_forces(mut self, apply_forces: bool) -> Self {
        self.apply_forces = apply_forces;

        self
    }

    pub fn attract(&self, mover: &mut Mover) {
        if mover.apply_forces {
            let mut force = self.pos.clone() - mover.pos.clone();
            let distance_sq = vec_math::mag_sq(&force).clamp(25., 2500.);

            let strength = G * (self.m * mover.m) / distance_sq;

            vec_math::set_mag(&mut force, strength);
            mover.apply_force(&force);
        }
    }
}