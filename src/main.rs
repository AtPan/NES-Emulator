pub mod processor;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod cpu;
pub mod gui;

use processor::*;
use register::*;

use fltk::prelude::*;
use fltk::enums::Color;
use fltk::{frame::*, app::*, window::*, button::*};

const WINDOW_WIDTH: i32 = 800;
const DEBUG_WINDOW_WIDTH: i32 = 400;
const DEBUG_WINDOW_TEXT_HEIGHT: i32 = 25;

fn main() {
    let proc = &mut Processor::new();

    proc.registers.a = RegisterChar(0x10);
    cpu::load_u8_memory(&mut proc.registers.sr, &mut proc.registers.a.0, 0x20);


    let app = App::default();
    let mut window = Window::new(100, 100, 800, 600, "Hello World");
    let mut inner_window = Window::default()
        .with_size(DEBUG_WINDOW_WIDTH, 600)
        .with_pos(WINDOW_WIDTH - DEBUG_WINDOW_WIDTH, 0);
    inner_window.set_color(Color::Black);
    let mut title_frame = Frame::default()
        .with_size(DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_TEXT_HEIGHT)
        .with_pos(0, 0)
        .with_label("=== 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
    title_frame.set_label_color(Color::White);
    let mut frame = Frame::default()
        .with_size(DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_TEXT_HEIGHT)
        .with_pos(0, DEBUG_WINDOW_TEXT_HEIGHT)
        .with_label("0000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00");
    frame.set_label_color(Color::White);

    inner_window.end();

    window.end();
    window.show();

    app.run().unwrap();
}
