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

    pub fn render(&mut self, grid: &[Vec<T>]) {
        self.draw_grid(grid);
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
}
