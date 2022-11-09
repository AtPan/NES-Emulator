use sdl2::{
    ttf,
    pixels::Color,
    video::Window,
    render::Canvas,
    rect::Rect,
};

use std::process;

pub struct DebugWindow<'a> {
    pub width: u32,
    pub height: u32,
    pub start: u32,
    pub font: ttf::Font<'a, 'static>,
    pub line_height: u32,
    pub lines: u32,
}

impl<'a> DebugWindow<'a> {
    pub fn new(width: u32, height: u32, start: u32, font_render: &'a ttf::Sdl2TtfContext, line_height: u32) -> Self {
        let font = match font_render.load_font("/home/antpan/Documents/Programming/emu/nes/fonts/ponde.ttf", 11) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error loading font: {}", e);
                process::exit(3);
            }
        };

        DebugWindow { width, height, start, font, line_height, lines: (height / line_height) }
    }

    pub fn render_line(&self, canvas: &mut Canvas<Window>, line: u32, text: String) {
        let surface = match self.font.render(&text).solid(Color::WHITE) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error rendering font: {}", e);
                process::exit(4);
            }
        };
        let width = surface.width();

        match canvas.texture_creator().create_texture_from_surface(surface) {
            Ok(texture) => {
                if let Err(e) = canvas.copy(&texture, None,
                    Rect::new(5 + self.start as i32, (self.line_height * line) as i32, width, self.line_height)) {

                    eprintln!("Error displaying text: {}", e);
                    process::exit(5);
                }
            },
            Err(e) => {
                eprintln!("Error rendering text: {}", e);
                process::exit(5);
            },
        };
    }
}
