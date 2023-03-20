mod maze;
use maze::*;

fn main() {
    println!("Hello, world!");
    // let m = Maze::new(20, 10);
    // m.print();

    let mut n = Node::new(&Pt::new(0, 0));
    let n1 = Node::new(&Pt::new(1, 0));
    n.children.push(n1);

    let n2 = n.clone();
    n2.print();
}
