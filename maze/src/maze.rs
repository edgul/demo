use rand::Rng;

pub struct Maze {
    width: usize,
    height: usize,
    tree: Node,
    table: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let start = Pt::new(0, height / 2);
        let end = Node::new(&Pt::new(width - 1, height / 2));

        let mut temp_self = Self {
            width,
            height,
            tree: Node::new(&start),
            table: vec![vec![Cell::new(); width]; height],
        };

        // add happy path to the tree
        let all_paths = temp_self.valid_paths(&start, &end.get_pt());
        let random_index = rand::thread_rng().gen_range(0..all_paths.len());
        temp_self.tree = all_paths[random_index].clone();

        // make new paths and connect to the tree
        let mut empty_cells = temp_self.empty_cells();
        while empty_cells.len() > 0 {
            // start from random empty cell
            let r_empty_cell_index = rand::thread_rng().gen_range(0..empty_cells.len());
            let empty_pt = empty_cells[r_empty_cell_index].clone();

            // grab random joinable tree node
            let joinable_pts = temp_self.joinable_nodes(&empty_pt);
            let random_index = rand::thread_rng().gen_range(0..joinable_pts.len());
            let tree_pt = joinable_pts[random_index].clone();

            // grab random path of all paths between start and tree node
            let paths = temp_self.valid_paths(&empty_pt, &tree_pt);
            let random_path_index = rand::thread_rng().gen_range(0..paths.len());
            let mount_node = temp_self.tree.get(tree_pt).unwrap();

            // add the path to the tree
            mount_node.add(paths[random_path_index].clone());
            empty_cells = temp_self.empty_cells();
        }

        // todo: populate the table

        temp_self
    }

    // todo
    fn valid_paths(&self, src: &Pt, dest: &Pt) -> Vec<Node> {
        let paths = vec![];
        paths
    }

    // todo
    fn joinable_nodes(&self, from_pt: &Pt) -> Vec<Pt> {
        let pts = vec![];
        pts
    }

    fn empty_cells(&mut self) -> Vec<Pt> {
        let mut v = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let pt = Pt::new(x as usize, y as usize);
                if self.tree.get(pt.clone()).is_none() {
                    v.push(pt);
                }
            }
        }
        v
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

    // seems to be bug in rust analyser(?),
    // derive Clone with pt.clone() calls compiles
    // implementing to shut the linter up
    pub fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pt: Pt,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(pt: &Pt) -> Self {
        Self {
            pt: Pt { x: pt.x, y: pt.y },
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
