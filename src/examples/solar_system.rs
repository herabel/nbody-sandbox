use egui_macroquad::egui::{vec2, Vec2};
use egui_macroquad::macroquad::color::*;

use crate::Body;

pub fn spawn_solar_system(bodies: &mut Vec<Body>, center: Vec2) {
    const G: f32 = 1.0;
    const SUN_MASS: f32 = 330_000.0;

    // 1 AU = 160 pix
    const AU: f32 = 160.0;

    bodies.push(Body::new(
        "Sun".to_string(),
        center,
        SUN_MASS,
        18.0,
        vec2(0.0, 0.0),
        vec2(0.0, 0.0),
        Color::new(1.0, 0.88, 0.2, 1.0),
    ));

    let solar_objects: [(&str, f32, f32, f32, f32, Color); 12] = [
        ("Mercury", 0.387, 0.055, 2.2, 0.8, Color::new(0.72, 0.70, 0.68, 1.0)),
        ("Venus",   0.723, 0.815, 3.4, 2.1, Color::new(0.92, 0.82, 0.55, 1.0)),
        ("Earth",   1.000, 1.000, 3.6, 3.6, Color::new(0.20, 0.60, 1.00, 1.0)),
        ("Mars",    1.524, 0.107, 2.6, 5.0, Color::new(0.90, 0.38, 0.22, 1.0)),

        ("Vesta",   2.362, 0.00004, 1.3, 1.4, Color::new(0.75, 0.68, 0.60, 1.0)),
        ("Ceres",   2.767, 0.00016, 1.6, 4.2, Color::new(0.80, 0.80, 0.82, 1.0)),
        ("Pallas",  2.772, 0.00003, 1.3, 2.8, Color::new(0.68, 0.70, 0.75, 1.0)),

        ("Jupiter", 5.204, 317.8, 9.0, 0.4, Color::new(0.85, 0.65, 0.45, 1.0)),
        ("Saturn",  9.582, 95.20, 7.5, 2.7, Color::new(0.92, 0.85, 0.60, 1.0)),
        ("Uranus", 19.201, 14.50, 5.2, 4.8, Color::new(0.50, 0.85, 0.90, 1.0)),
        ("Neptune", 30.047, 17.15, 5.0, 1.2, Color::new(0.25, 0.45, 0.95, 1.0)),

        ("Pluto",  39.482, 0.0022, 1.5, 3.9, Color::new(0.70, 0.62, 0.55, 1.0)),
    ];

    for (name, dist_au, mass, radius, angle, color) in solar_objects {
        let r = dist_au * AU;

        let pos = center + vec2(r * angle.cos(), r * angle.sin());

        let v_orbit = (G * SUN_MASS / r).sqrt();

        let velocity = vec2(-angle.sin(), angle.cos()) * v_orbit;

        bodies.push(Body::new(
            name.to_string(),
            pos,
            mass,
            radius,
            velocity,
            vec2(0.0, 0.0),
            color,
        ));
    }
}
