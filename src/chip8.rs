use crate::cpu::Cpu;
use sdl2::render::WindowCanvas;
use std::fs::{File};
use std::io::{Read, Result};

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
        }
    }

    pub fn load_program(&mut self, filename: String) -> Result<()> {
        let mut buf = [0; 3583];
        let mut file = File::open(filename)?;

        file.read(&mut buf)?;

        self.cpu.load_rom(buf).unwrap();
        self.cpu.bus.clear_screen();
        Ok(())
    }

    pub fn run_cycle(&mut self) {
        self.cpu.cycle();
    }

    pub fn render(&mut self, mut canvas: &mut WindowCanvas) {
        self.cpu.bus.screen.render(&mut canvas);
    }
}