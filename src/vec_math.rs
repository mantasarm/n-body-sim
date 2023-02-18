use std::ops::MulAssign;

use notan::math::Vec2;

pub fn mag_sq(v: &Vec2) -> f32 {
    v.x * v.x + v.y * v.y
}

pub fn set_mag(v: &mut Vec2, len: f32) {
    *v = v.clone().normalize_or_zero();
    v.mul_assign(len);
}

pub fn limit(v: &mut Vec2, limit: f32) {
    if v.x > limit {
        v.x = limit
    } else if v.x < -limit{
        v.x = -limit;
    }

    if v.y > limit {
        v.y = limit
    } else if v.y < -limit{
        v.y = -limit;
    }
}