use sdl2::{
    Sdl,
    video::Window,
    render::Canvas,
    EventPump,
    VideoSubsystem,
    event::Event,
    pixels::Color,
    rect::Rect,
};


pub struct View {
   pub context: Sdl,
   pub video: VideoSubsystem,
   pub canvas: Canvas<Window>,
   pub event: EventPump,
   pub debug_window: bool,
   pub scale: u32,
}

impl View {
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        if let Err(e) = self.canvas.fill_rect(Rect::new(0, 0, self.scale, self.scale)) {
            panic!("Error on render: {}", e);
        }
        self.canvas.present();
    }

    pub fn event_loop(&mut self) {
        'program_active: loop {
            for event in self.event.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'program_active,
                    _ => {},
                }
            }

            self.render();
        }
    }
}
