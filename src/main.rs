use macroquad::prelude::*;
use crate::segway::{init_segway, draw_segway};
use crate::environment::{init_environment, draw_environment};
use crate::gui::{init_gui, update_gui};
use macroquad::prelude::get_frame_time;

mod segway;
mod environment;
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
    let (mut segway, mut gui, mut pid_controller) = init_game(&environment);

    let back_color = Color::new(0.00, 0.43, 0.95, 1.00);

    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(back_color);

        update_game(&mut segway, &mut environment, &mut gui, &mut pid_controller);

        handle_input(&mut segway, &mut environment);

        draw_environment(&environment, &segway);
        draw_segway(&segway);

        update_gui(&mut gui, &segway, &pid_controller);

        next_frame().await;
    }
}

fn init_game(environment: &environment::Environment) -> (segway::Segway, gui::Gui, physics::PIDController) {
    (
        init_segway(environment),
        init_gui(),
        physics::PIDController::new(1.0, 0.5, 0.1),
    )
}

fn update_game(
    segway: &mut segway::Segway,
    environment: &mut environment::Environment,
    gui: &mut gui::Gui,
    pid_controller: &mut physics::PIDController,
) {
    let dt = get_frame_time();
    physics::update_game(segway, environment, pid_controller, dt);
    gui::update_gui(gui, segway, pid_controller);
}

fn handle_input(segway: &mut segway::Segway, environment: &mut environment::Environment) {
    let min_tilt_angle = -30.0_f32.to_radians();
    let max_tilt_angle = 30.0_f32.to_radians();

    if is_key_down(KeyCode::A) {
        segway.tilt_angle -= 0.01;
    }
    if is_key_down(KeyCode::D) {
        segway.tilt_angle += 0.01;
    }

    segway.tilt_angle = segway.tilt_angle.clamp(min_tilt_angle, max_tilt_angle);
}

