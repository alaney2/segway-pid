use macroquad::prelude::*;

pub struct Segway {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub wheel_radius: f32,
    pub wheel_thickness: f32,
}

impl Segway {
    pub fn update(&mut self, angular_acceleration: f32) {
        self.angular_acceleration = angular_acceleration;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
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
    }
}

pub fn draw_segway(segway: &Segway) {
    let body_height = 160.0;
    let handlebar_length = 20.0;

    let body_x = segway.x + segway.wheel_radius;
    let body_y = segway.y;
    
    draw_line(body_x, body_y - body_height, body_x, body_y, 6.0, GREEN);
    // draw_line(body_x - handlebar_length / 2.0, body_y - body_height, body_x + handlebar_length / 2.0, body_y - body_height, 2.0, GREEN);
    draw_circle_lines(segway.x, segway.y, segway.wheel_radius, segway.wheel_thickness, LIGHTGRAY);
}
