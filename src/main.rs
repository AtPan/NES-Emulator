pub mod processor;
pub mod opcode;
pub mod register;
pub mod cpu;
pub mod gui;
pub mod memory;

use crate::memory::*;

use std::process;

use processor::*;
use gui::*;

fn main() {
    let mut proc = Processor::<Bus>::new();
    if let Err(e) = proc.load_rom() {
        eprintln!("Error reading ROM into register: {}", e);
        process::exit(5);
    }

    let font_render = match sdl2::ttf::init() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error initializing ttf::init(): {}", e);
            process::exit(3);
        }
    };

    let mut view = match ViewBuilder::default()
        .with_scale(2)
        .as_debug(&font_render, 16)
        .build()
    {

        Ok(v) => v,
        Err(e) => {
            eprintln!("FATAL: Error on building window\n{}", e);
            process::exit(1);
        }
    };

    view.event_loop(proc);
}
