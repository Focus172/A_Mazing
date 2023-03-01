use image::DynamicImage;
use crate::node::MazeNode;
//use crate::TILE_SIZE;

pub struct Maze {
    pub width: u32,
    pub height: u32,
    pub empty: Vec<MazeNode>, 
    pub walls: Vec<MazeNode>,
    pub open: Vec<MazeNode>,
    pub closed: Vec<MazeNode>,
    //pub best: Vec<MazeNode>,
}

impl Maze {
    pub fn new(image: DynamicImage) -> Maze {

        let (walls, empty) : (Vec<MazeNode>, Vec<MazeNode>) =  Self::initialize_titles(&image);

        Maze {
            width: image.height(),
            height: image.width(),
            empty: empty,
            walls: walls,
            open: Vec::new(),
            closed: Vec::new(),
            //best: Vec::new(),
        }
    }

    // should only be called once
    pub fn find_and_pop_first_node(&mut self) -> MazeNode {
        self.empty.remove(0)
    }

    pub fn copy_last_node(&mut self) -> MazeNode {
        let last = self.empty.pop().unwrap();
        let copy = last.clone();
        self.empty.push(last);
        copy
    }

    fn initialize_titles(image: &DynamicImage) -> (Vec<MazeNode>, Vec<MazeNode>) {
        let rows = image.width() as i32;
        //let cols = image.width();
    
        let mut count = 0;

        let mut walls: Vec<MazeNode> = Vec::new();
        let mut empty: Vec<MazeNode> = Vec::new();
    
        image.to_rgb8().pixels().into_iter()
            .for_each(|&pixel| {
                let x: i32 = count % rows;
                let y: i32 = count / rows;
                count += 1;
    
                if pixel.0[0] == 0 {
                    walls.push(MazeNode::new(x, y));
                } else {
                    empty.push(MazeNode::new(x, y));
                }
                
            });

        (walls, empty)
    }
}