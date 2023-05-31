use crate::segway::Segway;
use crate::environment::Environment;
use crate::guy::Guy;

pub struct PIDController {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub integral: f32,
    pub prev_error: f32,
    pub derivative: f32,
}

impl PIDController {
    pub fn new(p: f32, i: f32, d: f32) -> Self {
        PIDController {
            p,
            i,
            d,
            integral: 0.0,
            prev_error: 0.0,
            derivative: 0.0,
        }
    }

    pub fn update(&mut self, error: f32, dt: f32) -> f32 {
        self.integral += error * dt;
        let integral_limit = 1.0;
        self.integral = self.integral.clamp(-integral_limit, integral_limit);

        self.derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        self.p * error + self.i * self.integral + self.d * self.derivative
    }
}

pub fn update_game(
    segway: &mut Segway,
    guy: &mut Guy,
    _environment: &mut Environment,
    pid_controller: &mut PIDController,
    dt: f32,
    _user_input: f32,
) {
    let desired_tilt_angle = 0.0;
    let error = guy.tilt_angle - desired_tilt_angle;
    let mut angular_acceleration = pid_controller.update(error, dt);
    
    let max_speed = 1000.0;
    let min_speed = -1000.0;
    if segway.speed >= max_speed && angular_acceleration > 0.0 {
        angular_acceleration = 0.0;
    } else if segway.speed <= min_speed && angular_acceleration < 0.0 {
        angular_acceleration = 0.0;
    }
    segway.update(angular_acceleration, dt, guy);
    guy.update(segway, dt);
}
