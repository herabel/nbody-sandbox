use std::f32::consts::PI;
use egui_macroquad::macroquad::rand::gen_range;
use egui_macroquad::egui::{vec2, Vec2};
use egui_macroquad::macroquad::color::*;

use crate::palette;
use crate::Body;

pub fn spawn_galaxy(bodies: &mut Vec<Body>, count: usize, center: Vec2) {
    const G: f32 = 1.0;
    let core_mass = 1_000_000.0;

    bodies.push(Body::new(
        "Core".to_string(),
        center,
        core_mass,
        15.0,
        vec2(0.0, 0.0),
        vec2(0.0, 0.0),
        YELLOW,
    ));

    for i in 0..count {
        let r = gen_range(60.0, 700.0);
        let angle = gen_range(0.0, 2.0 * PI);

        let pos = center + vec2(r * angle.cos(), r * angle.sin());

        let v_orbit = (G * core_mass / r).sqrt();
        let velocity = vec2(-angle.sin(), angle.cos()) * v_orbit;

        let color = match gen_range(0, 3) {
            0 => Color::new(0.8, 0.9, 1.0, 1.0),
            1 => Color::new(1.0, 1.0, 1.0, 1.0),
            _ => Color::new(0.6, 0.8, 1.0, 1.0),
        };

        bodies.push(Body::new(
            format!("Body-{}", i),
            pos,
            0.05,
            gen_range(1.5, 3.0),
            velocity,
            vec2(0.0, 0.0),
            color,
        ));
    }
}