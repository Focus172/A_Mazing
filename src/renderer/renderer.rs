use graphics::{rectangle, clear, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};
use crate::maze::Maze;
use image::DynamicImage;
use crate::TILE_SIZE;
use crate::solver::MazeSolverInterface;
use crate::node::MazeNode;
use graphics::Context;

pub struct Renderer {
    pub gl: GlGraphics,
    pub maze_interface: MazeSolverInterface,
    pub mazeImage: DynamicImage,
}

impl Renderer {
    pub fn new(opengl: OpenGL, image: DynamicImage) -> Renderer {
        let mut maze = Maze::new(image.clone());
    
        let maze_int = MazeSolverInterface::new(maze);

        Renderer {
            gl: GlGraphics::new(opengl),
            maze_interface: maze_int,
            mazeImage: image,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) -> Result<(), std::io::Error> {

        //const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for entity in &self.maze_interface.maze.walls {
                Self::render_obj(entity, &c, gl, BLACK.clone());
            }

            //for entity in &self.maze_interface.maze.empty {
            //    Self::render_obj(entity, &c, gl, WHITE.clone());
            //}

            for entity in &self.maze_interface.maze.open {
                Self::render_obj(entity, &c, gl, BLUE.clone());
            }

            for entity in &self.maze_interface.maze.closed {
                Self::render_obj(entity, &c, gl, GREEN.clone());
            }

            //for entity in &self.maze_interface.maze.best {
            //    render(entity, RED);
            //}
        });

        Ok(())
    }

    fn render_obj(node: &MazeNode, c: &Context, gl: &mut GlGraphics, color: [f32; 4]) {
        let SQUARE: [f64; 4] = rectangle::square(0.0, 0.0, TILE_SIZE as f64);

        let (x, y) = node.location;

        let transform = c
            .transform
            .trans(x, y);
            //.rot_rad(rotation);
            //.trans(-25.0, -25.0);

        // Draw a box rotating around the middle of the screen.
        rectangle(color, SQUARE, transform, gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) -> Result<(), Box<dyn std::error::Error>> {
        self.maze_interface.step()?;

        let _dt = args.dt;

        Ok(())
    }
}
