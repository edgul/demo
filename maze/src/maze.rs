pub struct Maze {
    width: usize,
    height: usize,
    tree: Option<Node>,
    table: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let temp_self = Self {
            width,
            height,
            tree: None,
            table: vec![vec![Cell::new(); width]; height],
        };

        // make happy path
        let start = Node::new(0, height / 2);
        let end = Node::new(width - 1, height / 2);

        let all_paths = temp_self.valid_paths(start.get_pt(), end.get_pt());

        // todo: connect all empty nodes to tree

        // todo: populate the table

        temp_self
    }

    fn valid_paths(&self, src: Pt, dest: Pt) -> Vec<Vec<Pt>> {
        let paths = vec![];
        // todo
        paths
    }

    fn joinable_nodes(&self) -> Vec<Pt> {
        let pts = vec![];
        // todo
        pts
    }

    pub fn print(&self) {
        println!("Maze::print()");
    }
}

#[derive(Clone)]
struct Cell {
    left: bool,
    up: bool,
    right: bool,
    down: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            left: false,
            up: false,
            right: false,
            down: false,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Pt {
    x: usize,
    y: usize,
}

impl Pt {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Node {
    pt: Pt,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            pt: Pt { x, y },
            children: vec![],
        }
    }

    pub fn add(&mut self, n: Node) {
        self.children.push(n);
    }

    pub fn get(&mut self, pt: Pt) -> Option<&mut Node> {
        if pt == self.pt {
            return Some(self);
        }

        for c in &mut self.children {
            if let Some(n) = c.get(pt.clone()) {
                return Some(n);
            }
        }
        None
    }

    pub fn get_pt(&self) -> Pt {
        self.pt.clone()
    }

    pub fn print(&self) {
        self.print_inner(0);
    }

    fn print_inner(&self, indent: usize) {
        let indent_string = " ";
        println!(
            "{}({},{})",
            indent_string.repeat(indent),
            self.pt.x,
            self.pt.y
        );
        for c in &self.children {
            c.print_inner(indent + 1);
        }
    }
}
