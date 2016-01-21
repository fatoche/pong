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
    const IMAGE_HEIGHT: i32 = 300;
    const IMAGE_WIDTH: i32 = 200;

    let mut display = jamkit::Graphics::init("test", DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32);
    let texture = jamkit::Texture::load(&display, "bild.png");

    let mut direction = Richtung::Stopp;
    let mut y = 0;
    const STEP: i32 = 5;

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

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&texture, None, [0, y, IMAGE_WIDTH, y + IMAGE_HEIGHT]);
        frame.finish();
    }
}
