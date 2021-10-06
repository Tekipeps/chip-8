pub mod chip8;
pub mod bus;
pub mod renderer;
pub mod keyboard;
pub mod speaker;
pub mod ram;
pub mod cpu;

use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::chip8::{Chip8};
use crate::renderer::{COLS, ROWS};

const SCALE: u32 = 15;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", (COLS * SCALE)as u32,  (ROWS * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    // let audio_subsystem = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut vm = Chip8::new();

    // vm.load_program("./test_opcode.ch8".to_string());
    // vm.load_program("./c8_test.c8".to_string());
    // vm.load_program("./games/TEST/IBM.ch8".to_string());

    vm.load_program("./games/TEST/C8PIC.ch8".to_string());
    vm.load_program("./games/TEST/Rocket2.ch8".to_string());

     loop {
         vm.render(&mut canvas);
         vm.handle_event(&mut event_pump);
         vm.run_cycle();

        ::std::thread::sleep(Duration::new(0,  1_000_000_000u32/60));
    }
}
