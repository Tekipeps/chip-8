extern crate sdl2;
use sdl2::render::WindowCanvas;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;

pub const ROWS: u32 = 32;
pub const COLS: u32 = 64;

pub struct Renderer {
    pub buffer: [u8; (ROWS*COLS) as usize],
    pub rows: u32,
    pub cols: u32,
    pub scale: u32,
    // pub width: usize,
    // pub height: usize,
}

impl Renderer{


    pub fn new() -> Self {
        let rows = ROWS;
        let cols = COLS;
        let scale = 15;
        Renderer {
            buffer: [0; (ROWS*COLS)as usize],
            rows,
            cols,
            scale,
            // width: (COLS * scale) as usize,
            // height: (ROWS * scale) as usize,
        }
    }
    // first bool for collision, second bool for pixel on/off
    pub fn set_pixel(&mut self, mut x: u32, mut y: u32) -> (bool, bool) {
        let mut isCollision = false;
        if x > COLS {
            x -= COLS;
            isCollision = true;
        } else if x < 0 {
            x += COLS;
            isCollision = true;
        }

        if y > ROWS {
            y -= ROWS;
            isCollision = true;
        } else if y < 0 {
            y += ROWS;
            isCollision = true;
        }


        let pl = (x + (y * COLS)) as usize;
        self.buffer[pl] ^= 1;
        (isCollision, self.buffer[pl] != 1)
    }
    pub fn draw_byte(&mut self, byt: u8, x: u8, y: u8) -> bool {
        let mut x_coord = x as u32;
        let mut y_coord = y as u32;
        let mut b = byt;
        let erased = false;

        for _ in 0..8 {
            x_coord %= self.cols;
            y_coord %= self.rows;

            let index = (x as u32 + (y as u32 * self.cols)) as usize;
            let bit = (b & 0b1000_0000) >> 7;
            let prev_val = self.buffer[index];
            self.buffer[index] ^= bit;

            if prev_val == 1 && self.buffer[index] == 0 {
              erased == true;
            }

            x_coord += 1;
            b <<= 1;
        }

        erased
    }
    pub fn clear(&mut self) {
        self.buffer = [0; (ROWS*COLS) as usize];
    }

    pub fn render (&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for i in 0..COLS*ROWS{
            let x =( i % COLS )* self.scale;
            let y = (i / COLS) * self.scale;
            if self.buffer[i as usize] != 0 {
                canvas.fill_rect(Rect::new(x as i32, y as i32, self.scale as u32, self.scale  as u32)).unwrap();
            }

        }
        canvas.present();
    }

    pub fn test_render (&mut self) {
        self.set_pixel(0, 0);
        self.set_pixel(5, 2);
        self.set_pixel(9, 7);
    }
}

