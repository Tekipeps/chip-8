pub mod chip8;
pub mod keyboard;
pub mod renderer;
pub mod speaker;
mod cpu;

use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::chip8::{Chip8, SCALE};
use crate::renderer::{COLS, ROWS};


pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", (COLS * SCALE)as u32,  (ROWS * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut vm = Chip8::new(&mut canvas);
    vm.cpu.load_from_file(String::from("./games/TEST/IBM.ch8")).unwrap();
    vm.cpu.display_file_content();

    'running: loop {
        vm.cpu.renderer.test_render();
        vm.cpu.renderer.render();

        let mut event_pump = sdl_context.event_pump().unwrap();

        vm.cpu.
        ::std::thread::sleep(Duration::new(0,  1_000_000_000u32/60));
    }
}
