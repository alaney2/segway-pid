use macroquad::prelude::*;
use crate::segway::{init_segway, draw_segway};
use crate::environment::{init_environment, draw_environment};
use crate::gui::{init_gui, draw_speedometer};
use crate::physics::PIDController;
use crate::guy::{init_guy, draw_guy};
use macroquad::prelude::get_frame_time;

mod segway;
mod environment;
mod guy;
mod physics;
mod gui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut environment = init_environment();
    let (mut segway, mut gui, mut guy, mut pid_controller) = init_game(&environment);


    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLUE);

        update_game(&mut segway, &mut guy, &mut environment, &mut gui, &mut pid_controller);

        handle_input(&mut segway, &mut guy, &mut environment);

        draw_environment(&environment, &segway);
        draw_guy(&guy, &segway);
        draw_segway(&segway);

        egui_macroquad::draw();

        next_frame().await;
    }
}

fn init_game(environment: &environment::Environment) -> (segway::Segway, gui::Gui, guy::Guy, physics::PIDController) {
    let segway = init_segway(&environment);
    let gui = init_gui();
    let guy = init_guy(&segway, &gui);
    (
        segway,
        gui,
        guy,
        PIDController::new(10.0, 1.0, 2.0),
    )
}

fn update_game(
    segway: &mut segway::Segway,
    guy: &mut guy::Guy,
    environment: &mut environment::Environment,
    gui: &mut gui::Gui,
    pid_controller: &mut physics::PIDController,
) {
    let dt = get_frame_time();
    physics::update_game(segway, guy, environment, pid_controller, dt, guy.tilt_angle, gui);
    gui::update_gui(gui, segway, guy, pid_controller);
    draw_speedometer(segway.speed, segway.angular_velocity);
}

fn handle_input(_segway: &mut segway::Segway, guy: &mut guy::Guy, _environment: &mut environment::Environment) {
    let min_tilt_angle = -10.0_f32.to_radians();
    let max_tilt_angle = 10.0_f32.to_radians();

    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        guy.tilt_angle -= 0.01;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right){
        guy.tilt_angle += 0.01;
    }

    guy.tilt_angle = guy.tilt_angle.clamp(min_tilt_angle, max_tilt_angle);
    guy.tilt_angle = (guy.tilt_angle * 100.0).round() / 100.0;
}

