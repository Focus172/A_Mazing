use crate::TILE_SIZE;

#[derive(Clone, PartialEq)]
pub struct MazeNode {
    pub cordinate: (i32, i32),
    pub location: (f64, f64),
    pub f_cost: Option<i32>,
    pub distanceTraveled: i32,
    //pub parent: Option<Box<MazeNode>>,
}

impl MazeNode {
    pub fn new(x: i32, y: i32) -> MazeNode { //parent: Option<MazeNode>
        MazeNode {
            cordinate: (x, y),
            location: ((TILE_SIZE * x) as f64, (TILE_SIZE * y) as f64),
            f_cost: Option::None,
            distanceTraveled: 0,
        }
    }

    
    pub fn calculate_and_update_f_cost(&mut self, parent_distance: i32, goal: &(i32, i32)) {
        let mut fCost: i32 = 0;

        fCost += parent_distance;

        let (x, y) = self.cordinate;

        let x_dist = (x - goal.0).abs();
        let y_dist = (y - goal.1).abs();

        if x_dist > y_dist {
            fCost += (y_dist) * 14;
            fCost += (x_dist - y_dist) * 10;
        } else {
            fCost += (x_dist) * 14;
            fCost += (y_dist - x_dist) * 10;
        }
        self.f_cost = Some(fCost);
        self.distanceTraveled = parent_distance + 10; 
    }
}
