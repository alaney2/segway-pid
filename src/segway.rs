use macroquad::prelude::*;

pub struct Segway {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
}

impl Segway {
    pub fn update(&mut self, angular_acceleration: f32) {
        self.angular_acceleration = angular_acceleration;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
    }
}

pub fn init_segway(environment: &crate::environment::Environment) -> Segway {
    let wheel_radius = 80.0;
    Segway {
        x: screen_width() / 2.0,
        y: screen_height() - environment.ground_height - wheel_radius,
        angle: 0.0,
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
    }
}

pub fn draw_segway(segway: &Segway) {
    let wheel_radius = 80.0;
    let body_height = 200.0;

    let handlebar_length = 20.0;

    draw_circle_lines(segway.x, segway.y, wheel_radius, 5.0, BLUE);

    let body_x = segway.x + wheel_radius;
    let body_y = segway.y;

    draw_line(body_x, body_y - body_height, body_x, body_y, 8.0, GREEN);

    draw_line(body_x - handlebar_length / 2.0, body_y - body_height, body_x + handlebar_length / 2.0, body_y - body_height, 2.0, GREEN);
}
