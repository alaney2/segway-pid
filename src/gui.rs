use crate::segway::Segway;
use crate::physics::PIDController;
use crate::guy::Guy;
use egui_macroquad::egui;
use egui::{
    plot::{HLine, Legend, Line, Plot, PlotBounds, PlotPoints},
    Align, Align2, Color32, Layout,
};
use macroquad::prelude::*;
use macroquad::text::draw_text;

use std::collections::VecDeque;

pub struct Gui {
    integral_history: VecDeque<f32>,
    error_history: VecDeque<f32>,
    derivative_history: VecDeque<f32>,
    pub ui_scale: f32,
}

pub fn init_gui() -> Gui {
    Gui {
        integral_history: VecDeque::new(),
        error_history: VecDeque::new(),
        derivative_history: VecDeque::new(),
        ui_scale: 1.0,
    }
}

pub fn update_gui(
    gui: &mut Gui,
    segway: &Segway,
    guy: &Guy,
    pid_controller: &mut PIDController,
) {
     gui.integral_history.push_back(pid_controller.integral);
     gui.error_history.push_back(pid_controller.prev_error);
     gui.derivative_history.push_back(pid_controller.derivative);
 
     if gui.integral_history.len() > 100 {
         gui.integral_history.pop_front();
     }
     if gui.error_history.len() > 100 {
         gui.error_history.pop_front();
     }
     if gui.derivative_history.len() > 100 {
         gui.derivative_history.pop_front();
     }
 
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Segway PID Controller")
            .anchor(Align2::LEFT_TOP, egui::emath::vec2(30. * gui.ui_scale, 30. * gui.ui_scale))
            .fixed_size(egui::emath::vec2(250., 200.))
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.add(
                        egui::Slider::new(&mut pid_controller.p, 0.0..=20.0).text("P ")
                    );
                    ui.add(
                        egui::Slider::new(&mut pid_controller.i, 0.0..=10.0).text("I ")
                    );
                    ui.add(
                        egui::Slider::new(&mut pid_controller.d, 0.0..=10.0).text("D ")
                    );

                });
                ui.label(format!("Integral: {:.2}", pid_controller.integral));
                ui.label(format!("Error: {:.2}", pid_controller.prev_error));
                ui.label(format!("Derivative: {:.2}", pid_controller.derivative));
                ui.label(format!("Angular Acceleration: {:.2} rad / s² ", segway.angular_acceleration));
                ui.label(format!("Tilt Angle: {:.2} rad ", guy.tilt_angle));
                ui.label(format!("Distance Traveled: {:.2} m ", segway.distance_traveled));
                ui.add(
                    egui::Slider::new(&mut gui.ui_scale, 0.5..=2.0).text("UI Scale")
                );
            });

        egui::Window::new("PID Controller Graphs")
            .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-30. * gui.ui_scale, 30. * gui.ui_scale))
            .fixed_size(egui::emath::vec2(300. * gui.ui_scale, 200. * gui.ui_scale))
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                Plot::new("pid_plot")
                    .legend(Legend::default().position(egui::plot::Corner::RightBottom))
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show_axes([false, false])
                    .show_background(false)
                    .show_x(false)
                    .show_y(false)
                    .show(ui, |plot_ui| {
                        plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                            [0., -1. * 1.1],
                            [100 as f64, 1. * 1.1],
                        ));
                        plot_ui.hline(HLine::new(0.).color(Color32::WHITE).width(1.));
                        plot_ui.line(
                            Line::new(
                                gui.integral_history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &v)| [i as f64, v as f64])
                                    .collect::<PlotPoints>(),
                            )
                            .width(2.)
                            .name("Integral"),
                        );
                        plot_ui.line(
                            Line::new(
                                gui.error_history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &v)| [i as f64, v as f64])
                                    .collect::<PlotPoints>(),
                            )
                            .width(2.)
                            .name("Error"),
                        );
                        plot_ui.line(
                            Line::new(
                                gui.derivative_history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &v)| [i as f64, v as f64])
                                    .collect::<PlotPoints>(),
                            )
                            .width(2.)
                            .name("Derivative"),
                        );

                    });
            });
    })
}

pub fn draw_speedometer(speed: f32, angular_velocity: f32) {
    let text = format!("Velocity: {:.1} m/s", speed);
    
    let text_width = text.len() as f32 * 10.0;
    let x = (screen_width() - text_width) * 2.0 / 5.0;
    let y = 60.0;

    draw_text(&text, x, y, 30.0, WHITE);

    let angular_text = format!("Angular Velocity: {:.1} rad/s", angular_velocity);

    let angular_text_width = angular_text.len() as f32 * 10.0;
    let angular_x = (screen_width() - angular_text_width) * 2.0 / 5.0;
    let angular_y = 100.0;

    draw_text(&angular_text, angular_x, angular_y, 30.0, WHITE);
}
