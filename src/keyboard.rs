use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::event::Event;
use std::process;

pub struct Keyboard {
    await_keypress: bool,
    pressed_keys: Vec<Option<u8>>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            await_keypress: false,
            pressed_keys: Vec::new(),
        }
    }

    pub fn await_keypress(&mut self) -> Option<u8> {
        match self.pressed_keys.first() {
            Some(a) => *a,
            None => None
        }
    }

    pub fn get_curr_key(&self) -> Option<u8> {
        match self.pressed_keys.first() {
            Some(x) => *x,
            None => None
        }
    }

    pub fn is_key_pressed(&self, key: u8 ) -> bool {
        self.pressed_keys.contains(&Some(key))
    }
    pub fn clear_keys(&mut self) {
        self.pressed_keys = Vec::new();
    }

    pub fn handle_event(&mut self, event_pump: &mut EventPump) {
        let mut keys = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::Kp1), .. } => {
                    keys.push(Some(0x1));
                },
                Event::KeyDown { keycode: Some(Keycode::Kp2), .. } => {
                    keys.push(Some(0x2));
                },
                Event::KeyDown { keycode: Some(Keycode::Kp3), .. } => {
                    keys.push(Some(0x3));
                },
                Event::KeyDown { keycode: Some(Keycode::Kp4), .. } => {
                    keys.push(Some(0xC));
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    keys.push(Some(0x4));
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    keys.push(Some(0x5));
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    keys.push(Some(0x6));
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    keys.push(Some(0xD));
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    keys.push(Some(0x7));
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    keys.push(Some(0x8));
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    keys.push(Some(0x9));
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    keys.push(Some(0xE));
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    keys.push(Some(0xA));
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    keys.push(Some(0x0));
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    keys.push(Some(0xB));
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    keys.push(Some(0xF));
                },

                _ => {}
            }
        }
        self.await_keypress = false;
        self.pressed_keys = keys;
    }
}