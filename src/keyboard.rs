use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::event::Event;

struct Key (Keycode, u8);

pub struct Keyboard {
    keymap: [Key; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keymap: [
                Key(Keycode::Kp1, 0x1),
                Key(Keycode::Kp2, 0x2),
                Key(Keycode::Kp3, 0x3),
                Key(Keycode::Kp4, 0xc),
                Key(Keycode::Q, 0x4),
                Key(Keycode::W, 0x5),
                Key(Keycode::E, 0x6),
                Key(Keycode::R, 0xD),
                Key(Keycode::A, 0x7),
                Key(Keycode::S, 0x8),
                Key(Keycode::D, 0x9),
                Key(Keycode::F, 0xE),
                Key(Keycode::Z, 0xA),
                Key(Keycode::X, 0x0),
                Key(Keycode::C, 0xB),
                Key(Keycode::V, 0xF),
            ],
        }
    }

    pub fn get_key_press(event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // break 'running
                    return;
                },
                _ => {}
            }
        }
    }
}