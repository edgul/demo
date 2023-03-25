mod maze;
use maze::*;

fn main() {
    let m = Maze::new(80, 20);
    m.print();
}
