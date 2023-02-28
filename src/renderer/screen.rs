mod app;

use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use app::App;

pub struct Screen {
    pub is_running: bool,
    pub width: u32,
    pub height: u32,
    pub window: GlutinWindow,
    pub app: App,
    events: Events,
}

impl Screen {
    pub fn new(opengl: OpenGL) -> Screen {
        Screen {
            is_running: true,
            width: 800,
            height: 800,
            window: WindowSettings::new("A_Mazing", [800, 800])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            // Create a new game and run it.
            app: App {
                gl: GlGraphics::new(opengl),
                rotation: 0.0,
            },
            events: Events::new(EventSettings::new()),
        }
    }

    pub fn update(&mut self) {

        let event = self.events.next(&mut self.window);

        if let Some(e) = event {
            if let Some(args) = e.render_args() {
                self.app.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.app.update(&args);
            }
        } else {
            self.is_running = false;
        }
    }
}