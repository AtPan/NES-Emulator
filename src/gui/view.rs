use std::process;
use std::fmt::Write;

use sdl2::{
    Sdl,
    video::Window,
    render::Canvas,
    EventPump,
    VideoSubsystem,
    event::Event,
    pixels::Color,
    rect::Rect,
    keyboard::Keycode, ttf::FontStyle,
};

use crate::{
    processor::Processor,
    bus::Bus,
};
use super::DebugWindow;

pub struct View<'a> {
   pub context: Sdl,
   pub video: VideoSubsystem,
   pub canvas: Canvas<Window>,
   pub event: EventPump,
   pub debug: Option<DebugWindow<'a>>,
   pub scale: u32,
   pub width: u32,
   pub height: u32,
   pub frame: [u32; 256 * 240],
}

pub const LINES_MOVED_PER_STROKE: u32 = 8;
pub const BYTES_MOVED_PER_STROKE: u32 = LINES_MOVED_PER_STROKE * 16;

impl View<'_> {
    pub fn event_loop(&mut self, processor: Processor) {
        let mut debug_mem_addr_start: u32 = 0;
        let mut debug_stack_addr_offset: u32 = 0;
        let lines = if let Some(debug_window) = &self.debug { debug_window.lines } else { 0 };
        let lines_for_debug_mem = lines - 8;

        'program_active: loop {
            let iter = self.event.poll_iter();
            for event in iter {
                match event {
                    Event::Quit { .. } => break 'program_active,
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        if debug_mem_addr_start + (lines_for_debug_mem * BYTES_MOVED_PER_STROKE) < processor.bus.len {
                            debug_mem_addr_start += BYTES_MOVED_PER_STROKE;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        if let Some(val) = debug_mem_addr_start.checked_sub(BYTES_MOVED_PER_STROKE) {
                            debug_mem_addr_start = val;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        if debug_stack_addr_offset + 64 < processor.bus.len {
                            debug_stack_addr_offset += 16;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        if let Some(val) = debug_stack_addr_offset.checked_sub(16) {
                            debug_stack_addr_offset = val;
                        }
                    },
                    _ => {},
                }
            }

            self.reset_screen();
            self.render();
            self.debug_render(&processor.bus, debug_mem_addr_start, debug_stack_addr_offset);
            self.canvas.present();
        }
    }

    pub fn render(&mut self) {
        // Render Main Screen
        for i in 0..256 {
            for j in 0..240 {
                let data = self.frame[(i * 240) + j];
                let r = (data & 0x00ff0000) >> 16;
                let g = (data & 0x0000ff00) >> 8;
                let b = data & 0x000000ff;
                self.canvas.set_draw_color(Color::RGB(r as u8, g as u8, b as u8));
                if let Err(e) = self.canvas.fill_rect(Rect::new(j as i32, i as i32, self.scale, self.scale)) {
                    eprintln!("Error on screen render: {}", e);
                    process::exit(2);
                }

            }
        }
    }

    pub fn debug_render(&mut self, bus: &Bus, mem_addr: u32, stack_addr: u32) {
        if let Some(debug_window) = &mut self.debug {
            if let Err(e) = self.canvas.draw_rect(Rect::new(debug_window.start as i32, 0, debug_window.width, debug_window.height)) {
                eprintln!("Error rendering rect: {}", e);
                process::exit(6);
            };

            // Dump Section of RAM
            debug_window.render_line(&mut self.canvas, 0, "Memory Dump".to_string());
            debug_window.render_line(&mut self.canvas, 1,
                "------ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string());

            for i in 2..debug_window.lines-6 {
                let slice_start = mem_addr + ((i - 2) * 16);
                let slice = bus.as_slice(slice_start);

                let mut text = String::new();
                if let Err(e) = write!(text, "{:05X}: ", slice_start) {
                    eprintln!("Error formatting debug text: {}", e);
                    process::exit(11);
                }

                for j in slice.iter().take(16) {
                    if let Err(e) = write!(text, "{:02x} ", j) {
                        eprintln!("Error formatting debug text: {}", e);
                        process::exit(12);
                    }
                }

                if i == debug_window.lines-7 {
                    debug_window.font.set_style(FontStyle::UNDERLINE);
                }
                debug_window.render_line(&mut self.canvas, i as u32, text);
                debug_window.font.set_style(FontStyle::NORMAL);
            }


            // Dump Stack
            debug_window.render_line(&mut self.canvas, debug_window.lines as u32 - 6, "Stack Dump".to_string());
            debug_window.render_line(&mut self.canvas, debug_window.lines as u32 - 5, "------ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string());
            for i in (debug_window.lines-4)..debug_window.lines {
                let slice_start = stack_addr as u32 + ((i + 4 - debug_window.lines) * 16);
                let slice = bus.as_slice(slice_start);

                let mut text = String::new();
                if let Err(e) = write!(text, "{:05X}: ", slice_start) {
                    eprintln!("Error formatting debug text: {}", e);
                    process::exit(11);
                }

                for j in slice.iter().take(16) {
                    if let Err(e) = write!(text, "{:02x} ", j) {
                        eprintln!("Error formatting debug text: {}", e);
                        process::exit(12);
                    }
                }

                debug_window.render_line(&mut self.canvas, i as u32, text);
            }
        }
    }

    pub fn reset_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn draw_byte(&mut self, color: u32, byte: u32) {
        self.frame[byte as usize] = color;
    }

    pub fn draw_square(&mut self, color: u32, byte: u32) {
        for i in 0..16 {
            for j in byte..(byte+16) {
                self.frame[(i * 240) + j as usize] = color;
            }
        }
    }
}
