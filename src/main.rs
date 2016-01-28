extern crate jamkit;

use jamkit::Event::*;
use jamkit::KeyState::*;

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
    const IMAGE_HEIGHT: i32 = 120;
    const IMAGE_WIDTH: i32 = 50;
    const BALL_RADIUS: i32 = 20;

    let mut display = jamkit::Graphics::init("test", DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32);
    let texture = jamkit::Texture::load(&display, "bild.png");

    let mut direction = Richtung::Stopp;
    let mut y = 0;
    const STEP: i32 = 5;

    // Ballvariables
    let mut ball_x = DISPLAY_WIDTH/2;
    let mut ball_y = DISPLAY_HEIGHT/2;
    let mut velocity_x = -5;
    let mut velocity_y = -3;

    'main: loop {
        for event in display.poll_events() {
            match event {
                Closed => break 'main,
                KeyboardInput(state, key) => direction = process_key_event(state, key),
                _ => (),
            }
        }
        const END: i32 = DISPLAY_HEIGHT - IMAGE_HEIGHT - STEP;
        match (direction, y) {
            (Richtung::Hoch, STEP...DISPLAY_HEIGHT) => y -= STEP,
            (Richtung::Runter, 0...END) => y += STEP,
            _ => (),
        }

        ball_x += velocity_x;
        ball_y += velocity_y;
        // reflect ball from walls
        const RIGHT_BORDER: i32 = DISPLAY_WIDTH - BALL_RADIUS;
        const LOWER_BORDER: i32 = DISPLAY_HEIGHT - BALL_RADIUS;
        match ball_x {
            BALL_RADIUS...RIGHT_BORDER => (),
            _ => velocity_x *= -1,
        }
        match ball_y {
            BALL_RADIUS...LOWER_BORDER => (),
            _ => velocity_y *= -1,
        }

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&texture, None, [0, y, IMAGE_WIDTH, y + IMAGE_HEIGHT]);
        frame.draw(&texture, None, [ball_x - BALL_RADIUS, ball_y - BALL_RADIUS,
                                    ball_x + BALL_RADIUS, ball_y + BALL_RADIUS]);
        frame.finish();
    }
}
