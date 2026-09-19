use std::fmt::Write;
use std::time::Instant;

mod palette;
mod examples;

use egui_macroquad::*;
use egui_macroquad::egui::{vec2, Vec2};
use egui_macroquad::macroquad::color::{Color, BLUE, WHITE, YELLOW};
use egui_macroquad::macroquad::shapes::{draw_circle, draw_line};
use egui_macroquad::macroquad::time::get_frame_time;
use egui_macroquad::macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use egui_macroquad::macroquad::camera::{set_camera, set_default_camera, Camera2D};
use egui_macroquad::macroquad::input::mouse_wheel;
use egui_macroquad::macroquad::text::{draw_multiline_text, draw_text};
use macroquad::color::RED;
use macroquad::time::get_fps;
use crate::palette::{ACCENT, DARK_GRAY};

use crate::examples::spawn_galaxy;
use crate::examples::solar_system;

pub struct Body {
    pub name: String,
    pos: Vec2,
    trail: Vec<Vec2>,
    mass: f32,
    radius: f32,
    velocity: Vec2,
    acceleration: Vec2,
    color: Color,
}

impl Body {
    pub fn new(name:String, pos: Vec2, mass: f32, radius: f32, velocity:Vec2, acceleration:Vec2,  color: Color) -> Self {
        Body{
            name,
            pos,
            trail: Vec::new(),
            mass,
            radius,
            velocity,
            acceleration,
            color,
        }
    }
}

pub fn leapfrog_halfkick(bodies: &mut Vec<Body>, dt:f32) {
    for body in bodies {
        body.velocity += body.acceleration * (0.5 * dt);
    }
}

pub fn leapfrog_drift(bodies: &mut Vec<Body>, dt:f32) {
    for body in bodies {
        body.pos += body.velocity * dt;
    }
}

pub fn compute_bodies(bodies: &mut Vec<Body>){
    const G: f32 = 1.0;
    let dt = 0.016;


    let bodies_count = bodies.len();


    leapfrog_halfkick(bodies,dt);
    leapfrog_drift(bodies, dt);

    for i in 0..bodies_count {
        bodies[i].acceleration = Vec2::ZERO;
        for j in 0..bodies_count {
            if i == j { continue; }

            let softening = 20.0;

            let delta = bodies[j].pos - bodies[i].pos;
            let dist_sq = delta.length_sq() + softening * softening;
            let dist = dist_sq.sqrt();
            let denom = dist * dist * dist;
            let acc_from_internal = delta * (G * bodies[j].mass / denom);

            bodies[i].acceleration += acc_from_internal;
        }
    }

    leapfrog_halfkick(bodies,dt);

    // Leapfrog algorithm or Velocity Verlet integration
    // plus probably Gravitational softening factor
}

#[macroquad::main("Physical simulation")]
async fn main() {
    let mut bodies: Vec<Body> = Vec::new();
    let dt = 0.016;

    let mut zoom: f32 = 1.0;
    let mut camera_pos = vec2(600.0, 300.0);

    /*
    bodies.push(Body::new("Earth".to_string(), vec2(900.0,700.0),1.0, 4.0, vec2(-20.56, 15.42), vec2(0.0,0.0),BLUE));
    bodies.push(Body::new("Sun".to_string(), vec2(600.0,300.0),330_000.0, 20.0, vec2(0.0,0.0), vec2(0.0,0.0),YELLOW));
     */

    // spawn_galaxy::spawn_galaxy(&mut bodies, 700, Vec2::ZERO);

    examples::solar_system::spawn_solar_system(&mut bodies, camera_pos);

    let mut draw_a_grid: bool = false;
    let mut grid_spacer: f32 = 50.0;
    let mut grid_thickness: f32 = 1.0;

    let mut body_info_size: f32 = 20.0;
    let mut show_a_body_name: bool = true;
    let mut show_a_body_velocity: bool = true;
    let mut show_a_body_mass: bool = true;
    let mut show_a_body_acceleration: bool = true;
    let mut show_a_body_coordinates: bool = true;
    let mut show_a_body_to_body_0: bool = true;


    // buffer to get rid of unnecessary allocations inside every body
    let mut body_info_string = String::with_capacity(256);

    loop {
        let start = Instant::now();
        camera_pos = bodies[0].pos;

        clear_background(palette::DARK_GRAY);

        let (_, wheel_y) = mouse_wheel();
        if wheel_y > 0.0 {
            zoom *= 1.1; // + 10%
        } else if wheel_y < 0.0 {
            zoom *= 0.9; // - 10%
        }
        zoom = zoom.clamp(0.01, 100.0);

        let camera = Camera2D {
            target: macroquad::prelude::vec2(camera_pos.x, camera_pos.y),
            zoom: macroquad::prelude::vec2(
                1.0 / (screen_width() / 2.0),
                1.0 / (screen_height() / 2.0),
            ) * zoom,
            ..Default::default()
        };

        set_camera(&camera);

        for body in &mut bodies {
            if body.trail.len() > 500 {
                body.trail.remove(0);
            }
            body.trail.push(body.pos);
        }

        if draw_a_grid {
            let (mut x, mut y)  = (0.0-screen_width()*100.0/2.0, 0.0-screen_height()*100.0/2.0);
            while x < screen_width()*100.0 {
                draw_line(
                    x,
                    y,
                    x,
                    screen_height()*100.0,
                    grid_thickness,
                    palette::SLATE_GRAY);
                x += grid_spacer;
            }
            x = 0.0-screen_height()*100.0/2.0;
            while y < screen_height()*100.0 {
                draw_line(
                    x,
                    y,
                    screen_width()*100.0,
                    y,
                    grid_thickness,
                    palette::SLATE_GRAY);
                y += grid_spacer;
            };
        }

        compute_bodies(&mut bodies);
        let body0_pos = bodies[0].pos;
        for body in &mut bodies {

            if show_a_body_to_body_0 {
                draw_line(body0_pos.x, body0_pos.y, body.pos.x, body.pos.y, grid_thickness, RED);
            }

            for window in body.trail.windows(5) {
                let current_dot = window[0];
                let past_dot = window[1];
                draw_line(past_dot.x, past_dot.y, current_dot.x, current_dot.y, 2.0, palette::BODY_PATH);
            };

            body_info_string.clear();

            if show_a_body_name {
                let _ = writeln!(body_info_string, "name: {}", body.name);
            }
            if show_a_body_coordinates {
                let _ = writeln!(body_info_string, "coordinates [x,y]: {}", body.pos);
            }
            if show_a_body_velocity {
                let _ = writeln!(body_info_string, "velocity [x,y]: {}", &body.velocity);
            }
            if show_a_body_acceleration {
                let _ = writeln!(body_info_string, "acceleration [x,y]: {}", body.acceleration);
            }
            if show_a_body_mass {
                let _ = writeln!(body_info_string, "mass: {:.1}", body.mass);
            }

            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
            draw_multiline_text(&body_info_string, body.pos.x+body.radius+25.0, body.pos.y-10.0, body_info_size, Some(1.0), WHITE);
        }

        set_default_camera();

        let elapsed = start.elapsed();


        ui(|egui_ctx| {
            egui::Window::new("Menu")
                .show(egui_ctx, |ui| {
                    ui.group(|ui| {
                        ui.checkbox(&mut draw_a_grid,"Draw a grid?");
                        ui.add(egui::Slider::new(&mut grid_thickness, 0.5..=5.0).text("grid thickness"));
                        ui.add(egui::Slider::new(&mut grid_spacer, 5.0..=100.0).text("grid spacer"));
                    });
                    ui.group(|ui| {
                        ui.add(egui::Slider::new(&mut body_info_size, 9.0..=40.0).text("info size"));
                        ui.checkbox(&mut show_a_body_name,"Show body name?");
                        ui.checkbox(&mut show_a_body_coordinates,"Show body coordinates?");
                        ui.checkbox(&mut show_a_body_velocity,"Show body velocity?");
                        ui.checkbox(&mut show_a_body_acceleration,"Show body acceleration?");
                        ui.checkbox(&mut show_a_body_mass,"Show body mass?");
                        ui.checkbox(&mut show_a_body_to_body_0,"Show body line to body 0?");
                    });
                    ui.separator();
                    ui.label(format!("Render: {} µs", elapsed.as_micros()));
                    ui.label(format!("FPS: {}", get_fps()));
                });
        });

        draw();


        next_frame().await;
    }
}
