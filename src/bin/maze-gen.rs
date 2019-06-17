use maze_gen::Maze;

fn main() {
    let mut maze = Maze::new(25, 25);
    maze.generate();
    println!("{}", maze);
}
