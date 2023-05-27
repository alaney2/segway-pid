use macroquad::prelude::*;

pub struct Environment {
    pub ground_height: f32,
}

pub fn init_environment() -> Environment {
    Environment {
        ground_height: 80.0,
    }
}

pub fn draw_environment(environment: &Environment) {
    let window_width = screen_width();
    let window_height = screen_height();

    draw_line(
        0.0,
        window_height - environment.ground_height,
        window_width,
        window_height - environment.ground_height,
        2.0,
        DARKGRAY,
    );

    let num_lines = 10;
    let line_spacing = window_width / (num_lines as f32 + 1.0);
    let line_height = 50.0;

    for i in 0..num_lines {
        let line_x = (i as f32 + 1.0) * line_spacing;
        draw_line(
            line_x,
            window_height - environment.ground_height,
            line_x,
            window_height - environment.ground_height - line_height,
            2.0,
            LIGHTGRAY,
        );
    }
}
