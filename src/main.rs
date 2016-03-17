extern crate jamkit;

use jamkit::Event::*;
use jamkit::KeyState::*;
use std::f32::*;

#[derive(Clone, Copy)]
enum Richtung {
    Hoch,
    Runter,
    Stopp,
}

fn process_key_event(state: jamkit::KeyState, key: jamkit::Key) -> Richtung {
    let key_event = (state, key);
    match key_event {
        (Pressed, jamkit::Key::W) => Richtung::Hoch,
        (Pressed, jamkit::Key::S) => Richtung::Runter,
        _ => Richtung::Stopp,
    }
}

fn main() {

    const DISPLAY_WIDTH: i32 = 640;
    const DISPLAY_HEIGHT: i32 = 480;
    const PADDLE_HEIGHT: i32 = 120;
    const PADDLE_WIDTH: i32 = 50;
    const BALL_RADIUS: i32 = 20;
    const START_X_VELOCITY: f32 = -5.0;
    const START_Y_VELOCITY: f32 = -3.0;
    const START_VELOCITY: f32 = 0.5;

    let mut display = jamkit::Graphics::init("test", DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32);
    let texture = jamkit::Texture::load(&display, "bild.png");

    let mut direction = Richtung::Stopp;
    let mut paddle_y = 0;
    const STEP: i32 = 8;

    // Ballvariables
    let mut ball_x = DISPLAY_WIDTH/2;
    let mut ball_y = DISPLAY_HEIGHT/2;
    let mut velocity_x = START_X_VELOCITY;
    let mut velocity_y = START_Y_VELOCITY;
    let mut vel = START_VELOCITY;

    'main: loop {
        for event in display.poll_events() {
            match event {
                Closed => break 'main,
                KeyboardInput(state, key) => direction = process_key_event(state, key),
                _ => (),
            }
        }
        const END: i32 = DISPLAY_HEIGHT - PADDLE_HEIGHT - STEP;
        match (direction, paddle_y) {
            (Richtung::Hoch, STEP...DISPLAY_HEIGHT) => paddle_y -= STEP,
            (Richtung::Runter, 0...END) => paddle_y += STEP,
            _ => (),
        }

        ball_x += (velocity_x * vel) as i32;
        ball_y += (velocity_y * vel) as i32;
        // reflect ball from walls
        const RIGHT_BORDER: i32 = DISPLAY_WIDTH - BALL_RADIUS;
        const LOWER_BORDER: i32 = DISPLAY_HEIGHT - BALL_RADIUS;
        const PADDLE_BORDER: i32 = PADDLE_WIDTH + BALL_RADIUS;
        match (ball_x, ball_y - paddle_y) {
            (0...PADDLE_BORDER, 0...PADDLE_HEIGHT) => {
                ball_x = PADDLE_BORDER;
                if velocity_y < BALL_RADIUS as f32 {
                    vel *= 1.1;
                } else {
                    vel = 1.0;
                }
                velocity_x *= -1.0;
                let dist = (ball_y - paddle_y - PADDLE_HEIGHT / 2).abs();
            }
            (x, _) if x < BALL_RADIUS => {
                ball_x = DISPLAY_WIDTH/2;
                ball_y = DISPLAY_HEIGHT/2;
                velocity_x = START_X_VELOCITY;
                velocity_y = START_Y_VELOCITY;
                vel = START_VELOCITY;
            }
            (BALL_RADIUS...RIGHT_BORDER, _) => (),
            _ => velocity_x *= -1.0,
        }
        match ball_y {
            BALL_RADIUS...LOWER_BORDER => (),
            _ => velocity_y *= -1.0,
        }

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&texture, None, [0, paddle_y, PADDLE_WIDTH, paddle_y + PADDLE_HEIGHT]);
        frame.draw(&texture, None, [ball_x - BALL_RADIUS, ball_y - BALL_RADIUS,
                                    ball_x + BALL_RADIUS, ball_y + BALL_RADIUS]);
        frame.finish();
    }
}
