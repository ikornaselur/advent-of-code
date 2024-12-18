use minifb::{Key, Window, WindowOptions};
use std::marker::PhantomData;

pub struct GridRenderer<T, F>
where
    F: Fn(&T) -> u32,
{
    window: Window,
    buffer: Vec<u32>,
    cell_size: usize,
    width: usize,
    height: usize,
    colour_mapper: F,
    _phantom: PhantomData<T>,
}

const FONT_5X7: [u64; 128] = {
    let mut chars = [0; 128];
    chars[b'0' as usize] = 0b01110_10001_10001_10001_10001_10001_01110;
    chars[b'1' as usize] = 0b00100_11100_00100_00100_00100_00100_11111;
    chars[b'2' as usize] = 0b01110_10001_00001_00010_00100_01000_11111;
    chars[b'3' as usize] = 0b01110_10001_00001_00110_00001_00001_01110;
    chars[b'4' as usize] = 0b00001_00011_00101_01001_11111_00001_00001;
    chars[b'5' as usize] = 0b11111_10000_10000_11110_00001_10001_01110;
    chars[b'6' as usize] = 0b01110_10001_10000_11110_10001_10001_01110;
    chars[b'7' as usize] = 0b11111_00001_00010_00100_01000_01000_01000;
    chars[b'8' as usize] = 0b01110_10001_10001_01110_10001_10001_01110;
    chars[b'9' as usize] = 0b01110_10001_10001_01111_00001_10001_01110;
    chars
};
const FONT_WIDTH: usize = 5;
const FONT_HEIGHT: usize = 7;
const FONT_SPACING: usize = 1;

impl<T, F> GridRenderer<T, F>
where
    F: Fn(&T) -> u32,
{
    pub fn new(width: usize, height: usize, cell_size: usize, colour_mapper: F) -> Self {
        let pixel_width = width * cell_size;
        let pixel_height = height * cell_size;

        let window = Window::new(
            "Grid Viewer",
            pixel_width,
            pixel_height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| panic!("{}", e));

        GridRenderer {
            window,
            buffer: vec![0; pixel_width * pixel_height],
            cell_size,
            width: pixel_width,
            height: pixel_height,
            colour_mapper,
            _phantom: PhantomData,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn render(&mut self, grid: &[Vec<T>], text: Option<(&str, usize, usize, u32, usize)>) {
        self.draw_grid(grid);

        if let Some((text, x, y, colour, scale)) = text {
            self.draw_text(text, x, y, colour, scale);
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    fn draw_grid(&mut self, grid: &[Vec<T>]) {
        self.buffer.fill(0);

        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let colour = (self.colour_mapper)(cell);
                self.draw_cell(x, y, colour);
            }
        }
    }

    fn draw_cell(&mut self, x: usize, y: usize, colour: u32) {
        let start_x = x * self.cell_size;
        let start_y = y * self.cell_size;

        for dy in 0..self.cell_size {
            for dx in 0..self.cell_size {
                let px = start_x + dx;
                let py = start_y + dy;
                if px < self.width && py < self.height {
                    self.buffer[py * self.width + px] = colour;
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, color: u32, scale: usize) {
        let mut cursor_x = x;
        let scale = scale.max(1);

        for c in text.bytes() {
            if cursor_x + FONT_WIDTH * scale >= self.width {
                break;
            }

            let bitmap = FONT_5X7[c as usize];

            // Draw each pixel of the character
            for cy in 0..FONT_HEIGHT {
                // Extract the current row (5 bits)
                let row = (bitmap >> (FONT_WIDTH * (FONT_HEIGHT - 1 - cy))) & 0b11111;

                // Process each bit in the row from left to right
                for cx in 0..FONT_WIDTH {
                    let bit = (row >> (FONT_WIDTH - 1 - cx)) & 1;
                    if bit == 1 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px = cursor_x + (cx * scale) + sx;
                                let py = y + (cy * scale) + sy;

                                if px < self.width && py < self.height {
                                    self.buffer[py * self.width + px] = color;
                                }
                            }
                        }
                    }
                }
            }

            cursor_x += (FONT_WIDTH + FONT_SPACING) * scale;
        }
    }
}
