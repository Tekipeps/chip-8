extern crate sdl2;
use sdl2::render::WindowCanvas;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;

pub const ROWS: i32 = 32;
pub const COLS: i32 = 64;

pub struct Renderer<'a> {
    pub buffer: [u8; (ROWS*COLS) as usize],
    pub canvas: &'a mut WindowCanvas,
    pub rows: i32,
    pub cols: i32,
    pub scale: i32,
    pub width: i32,
    pub height: i32,
}

impl <'a> Renderer<'a> {
    pub fn new(scale: i32, canvas: &'a mut WindowCanvas) -> Self {
        let rows = ROWS;
        let cols = COLS;
        Renderer {
            buffer: [0; (ROWS*COLS)as usize],
            rows,
            cols,
            scale,
            canvas,
            width: COLS*scale,
            height: ROWS*scale,
        }
    }
    pub fn set_pixel(&mut self, mut x: i32, mut y: i32) -> bool {
        if x > COLS {
            x -= COLS;
        } else if x < 0 {
            x += COLS;
        }

        if y > ROWS {
            y -= ROWS;
        } else if y < 0 {
            y += ROWS;
        }


        let pl = (x + (y * COLS)) as usize;
        self.buffer[pl] ^= 1;
        self.buffer[pl] != 1
    }

    pub fn clear(&mut self) {
        self.buffer = [0; (ROWS*COLS) as usize];
    }

    pub fn render (&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for i in 0..COLS*ROWS{
            let x =( i % COLS )* self.scale;
            let y = (i / COLS) * self.scale;
            if self.buffer[i as usize] != 0 {
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, self.scale as u32, self.scale  as u32)).unwrap();
            }

        }
        self.canvas.present();
    }

    pub fn test_render (&mut self) {
        self.set_pixel(0, 0);
        self.set_pixel(5, 2);
        self.set_pixel(9, 7);
    }
}

