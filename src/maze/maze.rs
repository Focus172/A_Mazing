use crate::node::NodeState;
use image::DynamicImage;
use crate::node::MazeNode;
use crate::TILE_SIZE;

pub struct Maze {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<MazeNode>, 
}

impl Maze {
    pub fn new(image: DynamicImage) -> Maze {
        Maze {
            width: image.height(),
            height: image.width(),
            tiles: initialize_titles(&image),
        }
    }

    pub fn _close_and_open_around(&mut self, mut to_close: MazeNode) { // this needs to take the real node to keep the compiler happy

        to_close.state = NodeState::Closed;

        let (cur_x, cur_y) = to_close.cordinate;

        self.tiles.iter_mut().for_each(|node| {
            match node.state {
                NodeState::Empty => {
                    if (cur_x+1, cur_y) == node.cordinate { node.state = NodeState::Open; }
                    if (cur_x-1, cur_y) == node.cordinate { node.state = NodeState::Open; }
                    if (cur_x, cur_y+1) == node.cordinate { node.state = NodeState::Open; }
                    if (cur_x, cur_y-1) == node.cordinate { node.state = NodeState::Open; }
                },
                _ => { }
            }
        });

        // the tile needs to have moved out of the vector to be in this method
        // so we can just do the good work of pushing it back in
        self.tiles.push(to_close);
    }

    pub fn find_first_node(&mut self) -> &mut MazeNode {
        self.tiles.iter_mut()
        .find(|node| {
            node.state == NodeState::Empty
        }).unwrap()
    }
}

fn initialize_titles(image: &DynamicImage) -> Vec<MazeNode> {
    let rows = image.height();
    //let cols = image.width();

    let mut count = 0;

    image.to_rgb8().pixels().into_iter()
        .map(|&pixel| {
            let x: u32 = (count % rows);
            let y: u32 = (count / rows);
            count += 1;

            //print!("{:?}", pixel);
            if pixel.0[0] == 0 {
                println!("this is a wall");
                MazeNode::new(x, y, NodeState::Wall)
            } else {
                println!("this is a empty");
                MazeNode::new(x, y, NodeState::Empty)
            }
            
        })
        .collect()
}

