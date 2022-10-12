use super::View;

use sdl2::{
    video::WindowBuildError,
    IntegerOrSdlError,
};

pub struct ViewBuilder {
    width: u32,
    height: u32,
    scale: u32,
    debug: bool,
}

impl Default for ViewBuilder {
    fn default() -> Self {
        ViewBuilder { width: 256, height: 240, scale: 1, debug: false }
    }
}

impl ViewBuilder {
    pub fn with_scale(mut self, scale: u32) -> Self {
        self.width *= scale;
        self.height *= scale;
        if self.debug {
            self.width += 200 * (scale - self.scale);
        }
        self.scale = scale;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.height = height * self.scale;
        self.width = width * self.scale;
        if self.debug {
            self.width += 200 * self.scale;
        }
        self
    }

    pub fn as_debug(mut self) -> Self {
        self.debug = true;
        self.width += 200 * self.scale;
        self
    }

    pub fn build(self) -> Result<View, String> {
        let context = match sdl2::init() {
            Ok(c) => c,
            Err(e) => return Err(format!("Error initializing SDL2: {}", e)),
        };
        let video = match context.video() {
            Ok(v) => v,
            Err(e) => return Err(format!("Error initializing SDL2 Video Subsystem: {}", e)),
        };
        let window = match video.window("Demo", self.width, self.height)
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
            context, video, canvas, event, debug_window: self.debug, scale: self.scale * 8,
        })

    }
}
