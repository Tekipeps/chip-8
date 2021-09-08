use sdl2::render::WindowCanvas;
use std::io::{Result};
use crate::cpu::Cpu;
use sdl2::AudioSubsystem;

pub const SCALE: i32 = 15;

pub struct Chip8<'a> {
    pub cpu: Cpu<'a>,
}

impl <'a> Chip8<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Self {
        Chip8 {
            cpu: Cpu::new(canvas)
        }
    }

}