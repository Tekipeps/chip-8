use crate::cpu::Cpu;
use sdl2::render::WindowCanvas;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
        }
    }

    pub fn load_program(&mut self, filename: String) {
        self.cpu.load_program(filename).unwrap();
        self.cpu.bus.clear_screen();
    }

    pub fn run_cycle(&mut self) {
        self.cpu.cycle();
    }

    pub fn render(&mut self, mut canvas: &mut WindowCanvas) {
        self.cpu.bus.screen.render(&mut canvas);
    }
}