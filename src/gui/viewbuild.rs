use super::{View, DebugWindow};

use sdl2::{
    video::WindowBuildError,
    IntegerOrSdlError,
    ttf,
};

pub const DEBUG_WINDOW_WIDTH: u32 = 255;

pub struct ViewBuilder<'a> {
    width: u32,
    height: u32,
    scale: u32,
    debug: Option<(&'a ttf::Sdl2TtfContext, u32)>,
}

impl Default for ViewBuilder<'_> {
    fn default() -> Self {
        ViewBuilder { width: 256, height: 240, scale: 1, debug: None }
    }
}

impl<'a> ViewBuilder<'a> {
    pub fn with_scale(mut self, scale: u32) -> Self {
        self.width *= scale;
        self.height *= scale;
        self.scale = scale;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.height = height * self.scale;
        self.width = width * self.scale;
        self
    }

    pub fn as_debug(mut self, font_render: &'a ttf::Sdl2TtfContext, line_height: u32) -> Self {
        self.debug = Some((font_render, line_height));
        self
    }

    pub fn build(self) -> Result<View<'a>, String> {
        let mut window_width: u32 = self.width;
        let window_height: u32 = self.height;
        let debug_window = match self.debug {
            Some((render, line_height)) => {
                window_width += DEBUG_WINDOW_WIDTH * self.scale;
                Some(DebugWindow::new(DEBUG_WINDOW_WIDTH * self.scale, self.height, self.width, render, line_height))
            },
            None => None,
        };

        let context = match sdl2::init() {
            Ok(c) => c,
            Err(e) => return Err(format!("Error initializing SDL2: {}", e)),
        };
        let video = match context.video() {
            Ok(v) => v,
            Err(e) => return Err(format!("Error initializing SDL2 Video Subsystem: {}", e)),
        };
        let window = match video.window("Demo", window_width, window_height)
            .position_centered()
            .build() {

            Ok(w) => w,
            Err(err) => match err {
                WindowBuildError::HeightOverflows(u) => return Err(format!("Height Overflow: {}", u)),
                WindowBuildError::WidthOverflows(u) => return Err(format!("Width Overflow: {}", u)),
                WindowBuildError::InvalidTitle(_) => return Err("Invalid Title".to_string()),
                WindowBuildError::SdlError(s) => return Err(s),
            }
        };
        let canvas = match window.into_canvas().build() {
            Ok(c) => c,
            Err(err) => match err {
                IntegerOrSdlError::IntegerOverflows(s, u) => return Err(format!("Integer Overflow: {} {}", s, u)),
                IntegerOrSdlError::SdlError(s) => return Err(s),
            }
        };
        let event = match context.event_pump() {
            Ok(e) => e,
            Err(err) => return Err(format!("Error initializing Event Pump: {}", err)),
        };

        Ok(View {
            context,
            canvas,
            event,
            video,
            debug: debug_window,
            height: self.height,
            width: self.width,
            scale: self.scale * 8,
            frame: [0; 256 * 240]
        })
    }
}
