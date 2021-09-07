use crate::renderer::Renderer;
use sdl2::render::WindowCanvas;

pub const SCALE: i32 = 15;

pub struct Chip8<'a> {
    pub renderer: Renderer<'a>,
}

impl <'a> Chip8<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Self {
        Chip8 {
            renderer: Renderer::new(15, canvas)
        }
    }
}