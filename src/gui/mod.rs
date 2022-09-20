use fltk::prelude::*;

use fltk::enums::{Color, Font};
use fltk::{app::*, window::*, text::*};

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 700;
pub const DEBUG_WINDOW_WIDTH: i32 = 425;

pub const DEBUG_MEMORY_DUMP_HEIGHT: i32 = 550;

static DEBUG_MEMORY_DUMP_TITLE: &str =
    "\t\t\tMemory Dump\n _____  00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f\n";
static DEBUG_STACK_DUMP_TITLE: &str =
    "\t\t\tStack Dump \n _____  00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f\n";

pub struct View {
    mem_buffer: TextBuffer,
    stk_buffer: TextBuffer,
    app: App,
}

impl Default for View {
    fn default() -> Self {
        Self {
            mem_buffer: TextBuffer::default(),
            stk_buffer: TextBuffer::default(),
            app: App::default(),
        }
    }
}

impl View {
    pub fn create_debug_window(self) -> Self {
        let mut screen = Window::new(100, 100, WINDOW_WIDTH + DEBUG_WINDOW_WIDTH, WINDOW_HEIGHT, "Debug");
        let mut debug_window = Window::default()
            .with_size(DEBUG_WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_pos(WINDOW_WIDTH, 0);
        debug_window.set_color(Color::Black);

        match Font::load_font("./fonts/PixeloidSans-nR3g1.ttf") {
            Ok(f) => Font::set_font(Font::Helvetica, &f),
            Err(_) => println!("Could not load font \"PixeloidSans-nR3g1.ttf\""),
        };

        let mut debug_memory_dump_text = TextDisplay::default()
            .with_size(DEBUG_WINDOW_WIDTH, DEBUG_MEMORY_DUMP_HEIGHT)
            .with_pos(0, 0);
        debug_memory_dump_text.set_color(Color::Black);
        debug_memory_dump_text.set_text_color(Color::White);
        debug_memory_dump_text.set_text_size(14);
        debug_memory_dump_text.set_text_font(Font::Helvetica);
        debug_memory_dump_text.set_buffer(self.mem_buffer.clone());

        let mut debug_stack_dump_text = TextDisplay::default()
            .with_size(DEBUG_WINDOW_WIDTH, WINDOW_HEIGHT - DEBUG_MEMORY_DUMP_HEIGHT)
            .with_pos(0, DEBUG_MEMORY_DUMP_HEIGHT);
        debug_stack_dump_text.set_color(Color::Black);
        debug_stack_dump_text.set_text_color(Color::White);
        debug_stack_dump_text.set_text_size(14);
        debug_stack_dump_text.set_text_font(Font::Helvetica);
        debug_stack_dump_text.set_buffer(self.stk_buffer.clone());

        debug_window.end();

        screen.end();
        screen.show();

        self
    }

    pub fn update_mem_buffer(&mut self, mem: &[u8], start: i32) {
        self.mem_buffer.set_text(DEBUG_MEMORY_DUMP_TITLE);
        for i in 0..20 {
            self.mem_buffer.append(&format!("{:04x}: ", start + (i * 16)));

            for b in mem.iter().take(16) {
                self.mem_buffer.append(&format!("{:02x} ", b));
            }

            self.mem_buffer.append("\n");
        }
    }

    pub fn update_stk_buffer(&mut self, mem: &[u8], start: i32) {
        self.stk_buffer.set_text(DEBUG_STACK_DUMP_TITLE);
        for i in (0..3).rev() {
            self.stk_buffer.append(&format!("{:04x}: ", start + (i * 16)));

            for b in mem.iter().take(16) {
                self.stk_buffer.append(&format!("{:02x} ", b));
            }

            self.stk_buffer.append("\n");
        }
    }

    pub fn run(&self) {
        //self.app.run().unwrap();
        while self.app.wait() {
        }
        println!("Exited");
    }
}
