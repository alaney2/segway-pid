use macroquad::prelude::*;
use crate::segway::{init_segway, draw_segway};
use crate::environment::{init_environment, draw_environment};
use crate::gui::{init_gui, update_gui};
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

    let back_color = Color::new(0.00, 0.43, 0.95, 1.00);

    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(back_color);

        update_game(&mut segway, &mut guy, &mut environment, &mut gui, &mut pid_controller);

        handle_input(&mut segway, &mut guy, &mut environment);

        draw_guy(&guy, &segway);
        draw_environment(&environment, &segway);
        draw_segway(&segway);

        // update_gui(&mut gui, &segway, &guy, &pid_controller);

        next_frame().await;
    }
}

fn init_game(environment: &environment::Environment) -> (segway::Segway, gui::Gui, guy::Guy, physics::PIDController) {
    let segway = init_segway(&environment);
    let guy = init_guy(&segway);
    (
        segway,
        init_gui(),
        // init_guy(segway),
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
    physics::update_game(segway, guy, environment, pid_controller, dt, guy.tilt_angle);
    gui::update_gui(gui, segway, guy, pid_controller);
}

fn handle_input(segway: &mut segway::Segway, guy: &mut guy::Guy, environment: &mut environment::Environment) {
    let min_tilt_angle = -12.0_f32.to_radians();
    let max_tilt_angle = 12.0_f32.to_radians();

    if is_key_down(KeyCode::A) {
        guy.tilt_angle -= 0.01;
    }
    if is_key_down(KeyCode::D) {
        guy.tilt_angle += 0.01;
    }

    guy.tilt_angle = guy.tilt_angle.clamp(min_tilt_angle, max_tilt_angle);
}

