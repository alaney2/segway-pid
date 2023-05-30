use crate::segway::Segway;
use crate::physics::PIDController;
use crate::guy::Guy;
use egui_macroquad::egui;
use egui::{
    epaint::Shadow,
    plot::{CoordinatesFormatter, Corner, HLine, Legend, Line, Plot, PlotBounds, PlotPoints},
    Align, Align2, Color32, Context, DragValue, Frame, Layout, Pos2, Slider, Vec2,
};
use macroquad::prelude::*;
use macroquad::text::draw_text;

pub struct Gui {
    pub integral: f32,
    pub error: f32,
    pub derivative: f32,
}

pub fn init_gui() -> Gui {
    Gui {
        integral: 0.0,
        error: 0.0,
        derivative: 0.0,
    }
}

pub fn update_gui(
    gui: &mut Gui,
    segway: &Segway,
    guy: &Guy,
    pid_controller: &mut PIDController,
) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Segway PID Controller")
            .anchor(Align2::LEFT_TOP, egui::emath::vec2(20., 20.))
            // .frame(Frame {
            //     inner_margin: egui::Margin::same(0.),
            //     outer_margin: egui::Margin::same(0.),
            //     rounding: egui::Rounding::none(),
            //     fill: Color32::TRANSPARENT,
            //     shadow: Shadow::NONE,
            //     stroke: egui::Stroke::NONE,
            // })
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.add(
                        egui::Slider::new(&mut pid_controller.p, 0.0..=20.0).text("P ")
                    );
                    ui.add(egui::Slider::new(&mut pid_controller.i, 0.0..=10.0).text("I "));
                    ui.add(egui::Slider::new(&mut pid_controller.d, 0.0..=10.0).text("D "));

                });
                ui.label(format!("Integral: {:.2}", pid_controller.integral));
                ui.label(format!("Error: {:.2}", gui.error));
                ui.label(format!("Derivative: {:.2}", gui.derivative));
                ui.label(format!("Angular Velocity: {:.2}", segway.angular_velocity));
                ui.label(format!("Angular Acceleration: {:.2}", segway.angular_acceleration));
                ui.label(format!("Tilt Angle: {:.2}", guy.tilt_angle));
            });
    });

    gui.integral = pid_controller.integral;
    gui.error = pid_controller.prev_error;
    gui.derivative = (gui.error - pid_controller.prev_error) / get_frame_time();
}


pub fn draw_speedometer(speed: f32) {
    let screen_width = screen_width();
    // let screen_height = screen_height();
    let text = format!("Velocity: {:.2} m/s", speed);
    let text_width = text.len() as f32 * 10.0;
    let x = (screen_width - text_width) / 2.0;
    let y = 50.0; // Top margin

    draw_text(&text, x, y, 30.0, WHITE);
}
