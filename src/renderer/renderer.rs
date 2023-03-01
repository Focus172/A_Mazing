use graphics::{rectangle, clear, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};
use crate::maze::Maze;
use image::DynamicImage;
use crate::TILE_SIZE;
use crate::solver::MazeSolverInterface;
use crate::node::NodeState;

pub struct Renderer {
    pub gl: GlGraphics,
    pub maze_interface: MazeSolverInterface,
    pub mazeImage: DynamicImage,
}

impl Renderer {
    pub fn new(opengl: OpenGL, image: DynamicImage) -> Renderer {
        let mut maze = Maze::new(image.clone());
        let mut first = maze.find_first_node();
        first.state = NodeState::Closed;

        let mut maze_int = MazeSolverInterface {
            maze,
            //current_node: first,
        };

        Renderer {
            gl: GlGraphics::new(opengl),
            maze_interface: maze_int,
            mazeImage: image,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) -> Result<(), std::io::Error> {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, TILE_SIZE as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for entity in &self.maze_interface.maze.tiles {
                let color = match entity.state {
                    NodeState::Empty => WHITE,
                    NodeState::Wall => BLACK,
                    NodeState::Closed => GREEN,
                    NodeState::Open => BLUE,
                    NodeState::Best => RED,
                };

                //let rotation = entity.rotation;
                let (x, y) = entity.location;

                let transform = c
                    .transform
                    .trans(x, y);
                    //.rot_rad(rotation);
                    //.trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                rectangle(color, square, transform, gl);
            }
        });

        Ok(())
    }

    pub fn update(&mut self, _args: &UpdateArgs) -> Result<(), Box<dyn std::error::Error>> {
        // Rotate 2 radians per second.

        self.maze_interface.step();

        //for _entity in &mut self.maze.tiles {
            //entity.location += 1.0 * args.dt;
            //entity.rotation += 2.0 * args.dt;
        //}

        Ok(())
    }
}