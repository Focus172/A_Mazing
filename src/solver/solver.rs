use crate::maze::Maze;
use crate::node::MazeNode; // how the fuck do I do this

pub struct MazeSolverInterface {
    pub maze: Maze,
    pub end: MazeNode,
}

impl MazeSolverInterface {
    pub fn new(mut maze: Maze) -> MazeSolverInterface {
        let mut start = maze.find_and_pop_first_node();
        let end = maze.copy_last_node();
        start.calculate_and_update_f_cost(-10, &end.cordinate);
        maze.open.push(start);
        
        MazeSolverInterface {
            maze,
            end,
        }
    }

    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>>{

        //println!("Step: {}", self.maze.open.len());
        // find the open node with the lowest F cost
        let mut should_pass = false;

        self.maze.closed.iter().for_each(|node| {
            if node.cordinate == self.end.cordinate {
                //println!("Found: {:?}", node.cordinate);
                should_pass = true;
            }
        });

        if should_pass {
            self.generate_finish_path();

            return Ok(());
        } else {
            let lowest_f_cost: MazeNode = self.pop_lowest_fcost_from_tiles();

            // open all the nodes around it
            self.close_and_open_around(lowest_f_cost);

            Ok(())
        }
    }

    // this method finds all the open nodes and compares their f_cost to the lowest
    // if it is less than the lowest, then it is poped from the tiles vector and the previous lowest is added back
    fn pop_lowest_fcost_from_tiles(&mut self) -> MazeNode {
        let mut lowest_f_cost_index: Option<usize> = Option::None;
        let mut lowest_f_cost_value: i32 = 10000000;

        for (index, node) in self.maze.open.iter().enumerate() {
            if let Some(cost) = node.f_cost {
                if cost < lowest_f_cost_value {
                    lowest_f_cost_value = cost;
                    lowest_f_cost_index = Some(index);
                }
            }
        }
        
        if let Some(index) = lowest_f_cost_index {
            return self.maze.open.remove(index);
        } else {
            todo!("No open nodes found. This should end the loop");
        }
    }

    fn generate_finish_path(&self) {

    }

    fn close_and_open_around(&mut self, to_close: MazeNode) { // this needs to take the real node to keep the compiler happy

        let point = to_close.cordinate;

        let mut to_change = Vec::new();

        self.maze.empty.iter().enumerate().for_each(|(index, node)| {
            if Self::transform(&point, (1, 0)) == node.cordinate { to_change.push(index); }
            if Self::transform(&point, (-1, 0)) == node.cordinate { to_change.push(index); }
            if Self::transform(&point, (0, 1)) == node.cordinate { to_change.push(index); }
            if Self::transform(&point, (0, -1)) == node.cordinate { to_change.push(index); }
        });

        let mut number_removed = 0;
        for index in to_change {
            let mut node = self.maze.empty.remove(index-number_removed);
            node.calculate_and_update_f_cost(to_close.distanceTraveled, &self.end.cordinate);
            self.maze.open.push(node);
            number_removed += 1;
        }

        // the tile needs to have moved out of the vector to be in this method
        // so we can just do the good work of pushing it back in
        self.maze.closed.push(to_close);
    }

    fn transform(point: &(i32, i32), other: (i32, i32)) -> (i32, i32) {
        (point.0 + other.0, point.1 + other.1)
    }
}