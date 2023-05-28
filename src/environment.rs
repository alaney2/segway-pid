use macroquad::prelude::*;
use std::f32::consts::PI;

pub struct Environment {
    pub ground_height: f32,
}

pub fn init_environment() -> Environment {
    Environment {
        ground_height: 80.0,
    }
}

pub fn draw_environment(environment: &Environment, segway: &crate::segway::Segway) {
    let window_width = screen_width();
    let window_height = screen_height();

    draw_line(
        0.0,
        window_height - environment.ground_height,
        window_width,
        window_height - environment.ground_height,
        6.0,
        LIGHTGRAY,
    );

    let num_lines = 10;
    let line_spacing = segway.wheel_radius * PI;
    let line_height = 20.0;
    let tilt_offset = 15.0;

    let line_offset = segway.distance_traveled % line_spacing;

    for i in -num_lines..num_lines {
        let line_x = (i as f32) * line_spacing - line_offset;
        draw_line(
            segway.x + line_x,
            window_height - environment.ground_height,
            segway.x + line_x - tilt_offset,
            window_height - environment.ground_height + line_height,
            4.0,
            LIGHTGRAY,
        );
    }
}

