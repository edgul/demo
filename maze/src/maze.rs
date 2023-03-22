use rand::Rng;

pub struct Maze {
    width: usize,
    //height: usize,
    table: Vec<Vec<Cell>>,
    explored_grid: Vec<Vec<bool>>,
    links: Vec<(Pt,Pt)>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let start = Pt::new(0, height / 2);
        // let end = Node::new(&Pt::new(width - 1, height / 2));

        let mut temp_self = Self {
            width,
            //height,
            table: vec![vec![Cell::new(); width]; height],
            explored_grid: vec![vec![false; width]; height],
            links: vec![],
        };

        temp_self.create_paths(&start);
        temp_self.populate_table();
        temp_self
    }

    fn right(src: &Pt, dest: &Pt) -> bool {
       src.x + 1 == dest.x && src.y == dest.y
    }
    fn down(src: &Pt, dest: &Pt) -> bool {
       src.x == dest.x && src.y + 1 == dest.y
    }

    fn adjacent(&self, pt: &Pt) -> Vec<Pt> {
        let mut v = Vec::new();
        if pt.x > 0 {
            v.push(Pt::new(pt.x - 1, pt.y));
        }
        if pt.x < self.explored_grid[0].len() - 1 {
            v.push(Pt::new(pt.x + 1, pt.y));
        }
        if pt.y > 0 {
            v.push(Pt::new(pt.x, pt.y - 1));
        }
        if pt.y < self.explored_grid.len() - 1 {
            v.push(Pt::new(pt.x, pt.y + 1));
        }
        v
    }

    fn adjacent_unexplored(&self, pt: &Pt) -> Vec<Pt> {
        let mut v = Vec::new();
        let adjacent_pts = self.adjacent(pt);
        for p in &adjacent_pts {
            if !self.explored_grid[p.y][p.x] {
                v.push(p.clone());
            }
        }
        v
    }

    #[cfg(Test)]
    fn overwrite_links_for_test(&mut self) {
        self.links = vec![];
        self.links.push((Pt::new(0,1), Pt::new(0,0)));
        self.links.push((Pt::new(0,1), Pt::new(1,1)));
        self.links.push((Pt::new(0,1), Pt::new(0,2)));
        self.links.push((Pt::new(0,2), Pt::new(0,3)));// shouldn't affect 
        
        self.links.push((Pt::new(2,1), Pt::new(2,0)));
        self.links.push((Pt::new(2,1), Pt::new(2,2)));
    }

    fn adjacent_explored(&self, pt: &Pt) -> Vec<Pt> {
        let mut v = Vec::new();
        let adjacent_pts = self.adjacent(pt);
        for p in &adjacent_pts {
            if self.explored_grid[p.y][p.x] {
                v.push(p.clone());
            }
        }
        v
    }

    #[cfg(Test)]
    fn print_explored_grid(&self) {
        for row in &self.explored_grid {
            for b in row {
                print!("{}", b);
            }
            println!("");
        }
    }

    fn create_paths(&mut self, src: &Pt) {
        let mut ready_for_exploring: Vec<Pt> = Vec::new();

        // init our exploration queue
        self.explored_grid[src.y][src.x] = true;
        ready_for_exploring.append(&mut self.adjacent_unexplored(src));

        while !ready_for_exploring.is_empty() {
            let ready_index = rand::thread_rng().gen_range(0..ready_for_exploring.len());
            let new_pt = ready_for_exploring.remove(ready_index);

            let previously_explored = self.adjacent_explored(&new_pt);
            let previous_index = rand::thread_rng().gen_range(0..previously_explored.len());
            let prev_pt = previously_explored[previous_index].clone();
            
            self.explored_grid[new_pt.y][new_pt.x] = true;
            self.links.push((prev_pt, new_pt.clone()));

            let adj_un = &mut self.adjacent_unexplored(&new_pt);
            for i in adj_un {
                let mut found = false;
                for j in &mut ready_for_exploring {
                    if i == j {
                        found = true;
                        break;
                    }
                }
                if !found { ready_for_exploring.push(i.clone()); }
            }
        }
    }

    #[cfg(Test)]
    fn print_links(&self) {
        for (pt1,pt2) in &self.links {
            println!("link: ({},{}), ({},{})", pt1.x, pt1.y, pt2.x,pt2.y);
        }
    }

    fn populate_table(&mut self) {
        // use links to populate the table
        for y in 0..self.table.len() {
            for x in 0..self.table[y].len() {
                let cell = &mut self.table[y][x];
                let cell_pt = Pt::new(x,y);
                for (pt1,pt2) in &self.links {
                    if &cell_pt == pt1 {
                        if Maze::right(&cell_pt, pt2) { cell.right = true; }
                        if Maze::down(&cell_pt, pt2) { cell.down = true; }
                    } else if &cell_pt == pt2 {
                        if Maze::right(&cell_pt, pt1) { cell.right = true; }
                        if Maze::down(&cell_pt, pt1) { cell.down = true; }
                    } 
                }
            }
        }
    }

    // todo: add start and end markings to drawing
    pub fn print(&self) {
        print!("-{}", "-".repeat(self.width*2-1)); // top border
        println!("-");
        for row in &self.table {
            print!("|"); // left border 1
            for (i, c) in row.iter().enumerate() {
                if i < row.len() {
                    if c.right { print!("  ")}
                    else { print!(" |");}
                }
            }
            println!("");
            print!("-"); // left border 2
            for c in row {
                if c.down { print!(" -")}
                else { print!("--");}
            }
            println!(""); // right border
        }
    }
}

#[derive(Clone)]
struct Cell {
    right: bool,
    down: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            right: false,
            down: false,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Pt {
    pub x: usize,
    pub y: usize,
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

