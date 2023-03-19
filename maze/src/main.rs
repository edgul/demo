mod maze;
use maze::*;

fn main() {
    println!("Hello, world!");
    let m = Maze::new(20, 10);
    m.print();

    let mut n = Node::new(0, 0);
    n.children.push(Node::new(0, 1));
    // probably excessive copying
    let mut n1 = Node::new(1, 0);
    n1.children.push(Node::new(1, 1));
    n.children.push(n1);
    n.print();

    let found = n.get(Pt::new(1, 0));
    found.unwrap().add(Node::new(2, 2));
    n.print();
}
