pub mod chip8;
pub mod cpu;
pub mod keyboard;
pub mod renderer;
pub mod speaker;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::chip8::{Chip8, SCALE};
use crate::renderer::{COLS, ROWS};


pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", (COLS * SCALE)as u32,  (ROWS * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut vm = Chip8::new(&mut canvas);

    'running: loop {
        vm.renderer.test_render();
        vm.renderer.render();

        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0,  1_000_000_000u32/60));
    }
}
