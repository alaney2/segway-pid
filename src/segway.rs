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
        self.speed = self.speed.clamp(-1000.0, 1000.0);
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
        speed: 200.0,
        distance_traveled: 0.0,
    }
}

pub fn draw_segway(segway: &Segway) {
    let body_height = 160.0;
    let handlebar_length = 20.0;

    let body_x = segway.x + segway.wheel_radius;
    let body_y = segway.y;

    let body_end_x = body_x + body_height / 8.0;
    let body_end_y = body_y - body_height / 2.0;

    let frame_end_x = body_x;
    let frame_end_y = body_y - body_height;
    
    // frame
    draw_line(body_x, body_y, body_end_x, body_end_y, 12.0, DARKGRAY);
    draw_line(body_end_x, body_end_y, frame_end_x, frame_end_y, 6.0, DARKGRAY);
    //handlebar
    draw_circle(frame_end_x - 2.0, frame_end_y, 6.0, DARKGRAY);
    // wheel
    draw_circle_lines(segway.x, segway.y, segway.wheel_radius, segway.wheel_thickness, LIGHTGRAY);
    draw_circle(segway.x, segway.y, segway.wheel_radius - segway.wheel_thickness / 2.0, DARKGRAY);

    // wheel line
    let line_length = segway.wheel_radius * 2.0 - segway.wheel_thickness;
    let adjusted_angle = segway.angle + PI / 2.0;
    let line_start_x = segway.x - line_length / 2.0 * adjusted_angle.cos();
    let line_start_y = segway.y - line_length / 2.0 * adjusted_angle.sin();
    let line_end_x = segway.x + line_length / 2.0 * adjusted_angle.cos();
    let line_end_y = segway.y + line_length / 2.0 * adjusted_angle.sin();

    draw_line(line_start_x, line_start_y, line_end_x, line_end_y, 6.0, LIGHTGRAY);

    // let adjusted_angle = segway.angle;
    // let line_start_x = segway.x - line_length / 2.0 * adjusted_angle.cos();
    // let line_start_y = segway.y - line_length / 2.0 * adjusted_angle.sin();
    // let line_end_x = segway.x + line_length / 2.0 * adjusted_angle.cos();
    // let line_end_y = segway.y + line_length / 2.0 * adjusted_angle.sin();

    // draw_line(line_start_x, line_start_y, line_end_x, line_end_y, 6.0, LIGHTGRAY);
}


