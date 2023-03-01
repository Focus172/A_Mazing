#![allow(dead_code)]

use crate::TILE_SIZE;

#[derive(PartialEq)]
pub enum NodeState {
    Empty, // used when a node is not a wall and has not been visited
    Wall, // used for walls
    Closed, // used when a node has been visited
    Open, // used when a node is allowed to be visited
    Best // only used for redering the final path
}

pub struct MazeNode {
    pub cordinate: (u32, u32),
    pub location: (f64, f64),
    pub state: NodeState,
    pub fCost: u32,
    distanceTraveled: u32,
    parent: Option<Box<MazeNode>>,
}

impl MazeNode {
    pub fn new(x: u32, y: u32, state: NodeState) -> MazeNode { //parent: Option<MazeNode>
        MazeNode {
            cordinate: (x, y),
            location: ((TILE_SIZE * x) as f64, (TILE_SIZE * y) as f64),
            state: state,
            fCost: 0,
            distanceTraveled: 0,
            parent: Option::None,
        }
    }

    
    fn calculateFCost(&self) -> u32 {
        let mut fCost: u32 = 0;

        if let Some(parent) = &self.parent {
            fCost += parent.distanceTraveled;

            let (x, y) = self.cordinate;
            //let (parentX, parentY) = Self::float_tuple_to_point(parent.location.to_tuple());

            if x > y {
                fCost += (y) * 14;
                fCost += (x - y) * 10;
            } else {
                fCost += (x) * 14;
                fCost += (y - x) * 10;
            }

            fCost
        } else {
            todo!();
        }    
    }

    /*
    fn openNode(xIndex: i32, yIndex: i32) {
        //THIS DECLARATION OF PARENT NODE IS WRONG, never mind i think it is right
        open.add(Node(xIndex, yIndex, closed[closed.size - 1], -1, -1)) //this doesnt work if there is nothing in closed
        val current = open[open.size - 1]
        current.fCost = calculateFCost(current)
        current.distanceTraveled = current.parent!!.distanceTraveled + 10
    }

    fn closeNode(current: Node) {
        closed.add(current)

        //check if current Objects.node is end

        //-----open adjacent if they arnt walls------
        //north
        var newOpenX = current.xIndex
        var newOpenY = current.yIndex - 1
        if (newOpenY > 0 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //east
        newOpenX = current.xIndex - 1
        newOpenY = current.yIndex
        if (newOpenX > 0 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //south
        newOpenX = current.xIndex
        newOpenY = current.yIndex + 1
        if (newOpenY < 65 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //west
        newOpenX = current.xIndex + 1
        newOpenY = current.yIndex
        if (newOpenX < 65 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }
        open.remove(current)
    }
    */
}
