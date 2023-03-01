use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use crate::renderer::Renderer;
use image::io::Reader as ImageReader;
use crate::TILE_SIZE;

pub struct Screen {
    pub is_running: bool,
    pub window: GlutinWindow,
    pub renderer: Renderer,
    events: Events,
}

impl Screen {
    pub fn new(opengl: OpenGL, image_path: &str) -> Result<Screen, Box<dyn std::error::Error>> {

        let image = ImageReader::open(image_path)?.decode()?;
        let width: u32 = TILE_SIZE * image.width();
        let height: u32 = TILE_SIZE * image.height();

        Ok(Screen {
            is_running: true,
            window: WindowSettings::new("A_Mazing", [width, height])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()?,
            // Create a new game and run it.
            renderer: Renderer::new(opengl, image),
            events: Events::new(EventSettings::new()),
        })
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let event = self.events.next(&mut self.window);

        if let Some(e) = event {
            if let Some(args) = e.render_args() {
                self.renderer.render(&args)?;
            }

            if let Some(args) = e.update_args() {
                self.renderer.update(&args)?;
            }
        } else {
            self.is_running = false;
        }

        Ok(())
    }
}