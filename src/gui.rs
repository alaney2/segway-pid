use crate::segway::Segway;
use crate::physics::PIDController;
use egui_macroquad::egui;
use macroquad::prelude::*;

pub struct Gui {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub integral: f32,
    pub error: f32,
    pub derivative: f32,
}

pub fn init_gui() -> Gui {
    Gui {
        p: 0.0,
        i: 0.0,
        d: 0.0,
        integral: 0.0,
        error: 0.0,
        derivative: 0.0,
    }
}

pub fn update_gui(gui: &mut Gui, segway: &Segway, pid_controller: &PIDController) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Segway PID Controller")
            .show(ctx, |ui| {
                ui.label(format!("P: {:.2}", pid_controller.p));
                ui.label(format!("I: {:.2}", pid_controller.i));
                ui.label(format!("D: {:.2}", pid_controller.d));
                ui.label(format!("Integral: {:.2}", pid_controller.integral));
                ui.label(format!("Error: {:.2}", gui.error));
                ui.label(format!("Derivative: {:.2}", gui.derivative));
            });
    });

    gui.p = pid_controller.p;
    gui.i = pid_controller.i;
    gui.d = pid_controller.d;
    gui.integral = pid_controller.integral;
    gui.error = pid_controller.prev_error;
    gui.derivative = (gui.error - pid_controller.prev_error) / get_frame_time();

    egui_macroquad::draw();
}
