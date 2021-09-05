pub mod modules;
use crate::modules::{Screen, VM, ScreenProps};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::ops::Index;
use sdl2::rect::Rect;


pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", Screen::COLS as u32 * vm.screen.scale ,  Screen::ROWS as u32 * scale )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut vm = VM::new(&sdl_context, &canvas);
    vm.display_screen();
    vm.screen.paint(5, 6);

    'running: loop {
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

        vm.screen.clear_screen();

        for i in 0..Screen::COLS*Screen::ROWS{
            let x =( i % Screen::COLS )* vm.screen.scale as usize;
            let y = (i / Screen::COLS) * vm.screen.scale as usize;
            if vm.screen.buffer[i] != 0x0 {
                canvas.fill_rect(Rect::new(x as i32, y as i32, vm.screen.scale as u32, vm.screen.scale as u32));
            }

        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
