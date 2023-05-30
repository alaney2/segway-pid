use macroquad::prelude::*;

pub struct Guy {
  pub tilt_angle: f32,
  pub start_x: f32,
  pub start_y: f32,
  pub end_x: f32,
  pub end_y: f32,
  pub height: f32,
}

impl Guy {
  pub fn update(&mut self, segway: &crate::segway::Segway, _delta_time: f32) {
    self.start_x = segway.x - segway.wheel_radius / 2.0 + self.height * self.tilt_angle.sin();
    self.start_y = segway.y - self.height * self.tilt_angle.cos();
  }
}

pub fn init_guy(segway: &crate::segway::Segway) -> Guy {
  let height = 220.0;
  Guy {
    tilt_angle: 0.0,
    start_x: segway.x - segway.wheel_radius / 2.0,
    start_y: segway.y - height,
    end_x: segway.x - segway.wheel_radius / 2.0,
    end_y: segway.y,
    height: height,
  }
}

pub fn draw_guy(guy: &Guy, segway: &crate::segway::Segway) {
  let segway_height = 160.0;
  draw_line(guy.start_x, guy.start_y, guy.end_x, guy.end_y, 10.0, LIGHTGRAY);

  // Arm:
  let arm_length = 42.0;
  if guy.tilt_angle == 0.0 {
    // no joint
    // draw_line(guy.start_x, segway.y - segway_height, segway.x + segway.wheel_radius, segway.y - segway_height, 6.0, LIGHTGRAY);

    // joint
    let midpoint_x = (guy.start_x + segway.x + segway.wheel_radius) / 2.0;

    let half_x = (segway.x + segway.wheel_radius - guy.start_x) / 2.0;
    let acos_angle = (half_x / arm_length).acos();

    draw_line(guy.start_x, segway.y - segway_height, midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), 6.0, LIGHTGRAY);
    draw_circle(midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), 2.0, LIGHTGRAY);
    draw_line(midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), segway.x + segway.wheel_radius, segway.y - segway_height, 6.0, LIGHTGRAY);

  } else {

    let m = (guy.end_y - guy.start_y) / (guy.end_x - guy.start_x);
    let b = guy.start_y - m * guy.start_x;
    let midpoint_x = ((((segway.y - segway_height) - b) / m) + segway.x + segway.wheel_radius) / 2.0;

    let half_x = (segway.x + segway.wheel_radius - (((segway.y - segway_height) - b) / m)) / 2.0;
    let acos_angle = (half_x / arm_length).acos();

    if acos_angle.is_nan() {
      draw_line(((segway.y - segway_height) - b) / m, segway.y - segway_height, segway.x + segway.wheel_radius, segway.y - segway_height, 6.0, LIGHTGRAY);
    }

    draw_line(((segway.y - segway_height) - b) / m, segway.y - segway_height, midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), 6.0, LIGHTGRAY);
    draw_circle(midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), 2.0, LIGHTGRAY);
    draw_line(midpoint_x, segway.y - segway_height + arm_length * acos_angle.sin(), segway.x + segway.wheel_radius, segway.y - segway_height, 6.0, LIGHTGRAY);
    
    // draw_line(((segway.y - segway_height) - b) / m, segway.y - segway_height, segway.x + segway.wheel_radius, segway.y - segway_height, 6.0, LIGHTGRAY);
  }

}