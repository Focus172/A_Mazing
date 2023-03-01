use crate::maze::Maze;
use crate::node::NodeState; // how the fuck do I do this

pub struct MazeSolverInterface {
    pub maze: Maze,
}

impl MazeSolverInterface {
    pub fn new(maze: Maze) -> MazeSolverInterface {
        MazeSolverInterface {
            maze,
        }
    }

    pub fn step(&mut self) {
        // find the open node with the lowest F cost
        self.maze.tiles.iter_mut().for_each(|node| {
            match node.state {
                NodeState::Open => {
                    // if it's the end node, stop
                    // if it's not the end node, add it to the closed list
                    // open all the nodes around it
                },
                _ => { }
            }
        });

        // add it to the closed list

        // open all the nodes around it

    }

    pub fn solve(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
        
/*
struct Pathfinder {
    startNodeX: u16,
    endNodeX: u16,
    endNodeY: u16,
    walls: Maze
}

impl Pathfinder {
    fn new() -> Pathfinder {
        Pathfinder {
            startNodeX: 0,
            endNodeX: 0,
            endNodeY: 0,
            walls: Maze::new()
        }
    }

    fn step() {
        //add the correct nodes IN ORDER to the path array
        println("OH Yeah!!!!!!!!!!!!!!!!!!!!!!");
        let overflowCount = 99;
        let i = 1;
        while (i < overflowCount) {
            let ind = indexOfLowestFCost();
            closeNode(open[ind]);
            println(closed[closed.size - 1].xIndex.toString() + " " + closed[closed.size - 1].yIndex);
            println("$endNodeX $endNodeY");
            println("");
            if (closed[closed.size - 1].xIndex == endNodeX && closed[closed.size - 1].yIndex == endNodeY) {
                i = overflowCount;
                println("path found!");
            }
            i += 1;
        }
        println("OH NO!!!!!!!!!!!!!!!!!!!!!!");
    }

    fn getLowestFCost() -> Node {
        var indexOfLowestFCost = 0
        for (i in open.indices) {
            if (open[i].fCost < open[indexOfLowestFCost].fCost) {
                indexOfLowestFCost = i
            }
        }
        return indexOfLowestFCost
    }

    fn checkIfOpen(xIndex: Int, yIndex: Int) -> Boolean { //there has to be a better way to do this
        for (current in open) { //need to also check closed
            if (current.xIndex == xIndex && current.yIndex == yIndex) {
                return false
            }
        }
        return true
    }
}
*/
    }
}