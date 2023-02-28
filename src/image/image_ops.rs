
fn read_image() {
    let image = ImageHelper::readImage("maze.png", 800, 800).unwrap();
    let mazeWall = ImageHelper::convertToBoolean(image);
    ImageHelper::printMaze(mazeWall)
}

fn print_maze(maze: Maze) {
    for r in maze_wall.indices() {
        for c in maze_wall[r].indices() {
            if maze_wall[r][c] {
                print!("  ");
            } else {
                print!("# ");
            }
        }
        println!();
    }
}

fun convertToBoolean(image: BufferedImage): Array<BooleanArray> {
    val width = image.width
    val height = image.height
    val result = Array(height) { BooleanArray(width) }
    for (r in 0 until height) {
        for (c in 0 until width) {
            if (image.getRGB(c, r) == -1) {
                result[r][c] = true
            } else result[r][c] = false
        }
    }
    return result
}
