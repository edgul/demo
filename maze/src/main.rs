mod maze;
use maze::*;

fn main() {
    let m = Maze::new(20, 10);
    m.print();
}
