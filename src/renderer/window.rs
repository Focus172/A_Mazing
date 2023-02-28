#[path ="../solver/solver.rs"]
mod solver;


use glutin_window::GlutinWindow;
use solver::MazeSolverInterface;

struct MazeWindow {
    solver: MazeSolverInterface,
    window: GlutinWindow,
}