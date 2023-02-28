struct MazeNode {
    x_index: usize,
    y_index: usize,
    parent: Option<Box<MazeNode>>,
    distance_traveled: usize,
    f_cost: usize,
}

impl MazeNode {
    fn new(x_index: usize, y_index: usize, parent: Option<Box<MazeNode>>, distance_traveled: usize, f_cost: usize) -> MazeNode {
        MazeNode {
            x_index,
            y_index,
            parent,
            distance_traveled,
            f_cost,
        }
    }
}