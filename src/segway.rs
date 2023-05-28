use macroquad::prelude::*;
use std::f32::consts::PI;

pub struct Segway {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub wheel_radius: f32,
    pub wheel_thickness: f32,
    pub speed: f32,
    pub tilt_angle: f32,
    pub distance_traveled: f32,
}

impl Segway {
    pub fn update(&mut self, angular_acceleration: f32, delta_time: f32) {
        self.angular_acceleration = angular_acceleration;
        self.angular_velocity += self.angular_acceleration;

        // let friction = 0.99;
        // self.speed *= friction;
        self.distance_traveled += self.speed * delta_time;

        self.angular_velocity = self.speed / self.wheel_radius;
        self.speed += self.angular_acceleration * delta_time;
        self.angle = (self.angle - self.angular_velocity * delta_time) % (2.0 * PI);
    }
}

pub fn init_segway(environment: &crate::environment::Environment) -> Segway {
    let wheel_radius = 40.0;
    let wheel_thickness = 8.0;
    Segway {
        x: screen_width() / 2.0,
        y: screen_height() - environment.ground_height - wheel_radius - wheel_thickness / 2.0,
        angle: 0.0,
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        wheel_radius,
        wheel_thickness,
        speed: 100.0,
        tilt_angle: 0.0,
        distance_traveled: 0.0,
    }
}

pub fn draw_segway(segway: &Segway) {
    let body_height = 160.0;
    let handlebar_length = 20.0;

    let body_x = segway.x + segway.wheel_radius;
    let body_y = segway.y;

    let frame_end_x = body_x + body_height * segway.tilt_angle.sin();
    let frame_end_y = body_y - body_height * segway.tilt_angle.cos();
    
    // frame
    draw_line(body_x, body_y, frame_end_x, frame_end_y, 6.0, GREEN);
    // wheel
    draw_circle_lines(segway.x, segway.y, segway.wheel_radius, segway.wheel_thickness, LIGHTGRAY);

    // wheel line
    let line_length = segway.wheel_radius * 2.0 - segway.wheel_thickness;
    let adjusted_angle = segway.angle + PI / 2.0;
    let line_start_x = segway.x - line_length / 2.0 * adjusted_angle.cos();
    let line_start_y = segway.y - line_length / 2.0 * adjusted_angle.sin();
    let line_end_x = segway.x + line_length / 2.0 * adjusted_angle.cos();
    let line_end_y = segway.y + line_length / 2.0 * adjusted_angle.sin();

    draw_line(
        line_start_x,
        line_start_y,
        line_end_x,
        line_end_y,
        6.0,
        GREEN,
    );
}


