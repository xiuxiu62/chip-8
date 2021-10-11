use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::{pixels::Color, EventPump};
use sdl2::{render::Canvas, video::Window};

use crate::{DISPLAY_HEIGHT, DISPLAY_LEN, DISPLAY_WIDTH};

pub struct Display {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub scale: u32,
    pub fgcol: Color,
    pub bgcol: Color,
}

impl Display {
    pub fn new(
        sdl_context: &Sdl,
        window_title: &str,
        scale: u32,
        fg_col: (u8, u8, u8),
        bg_col: (u8, u8, u8),
    ) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem
            .window(
                window_title,
                DISPLAY_WIDTH as u32 * scale,
                DISPLAY_HEIGHT as u32 * scale,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            event_pump,
            scale,
            fgcol: Color::RGB(fg_col.0, fg_col.1, fg_col.2),
            bgcol: Color::RGB(bg_col.0, bg_col.1, bg_col.2),
        }
    }

    // Clears the display to black
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.bgcol);
        self.canvas.clear();
        self.canvas.present();
    }

    // Renders the given buffer to the display
    pub fn render(&mut self, buffer: [u8; DISPLAY_LEN]) {
        // Fill with buffer
        let scl = self.scale as usize;
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                if buffer[y * DISPLAY_WIDTH + x] > 0 {
                    // Foreground
                    self.canvas.set_draw_color(self.fgcol);
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * scl) as i32,
                            (y * scl) as i32,
                            scl as u32,
                            scl as u32,
                        ))
                        .unwrap();
                } else {
                    // Background
                    self.canvas.set_draw_color(self.bgcol);
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * scl) as i32,
                            (y * scl) as i32,
                            scl as u32,
                            scl as u32,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
