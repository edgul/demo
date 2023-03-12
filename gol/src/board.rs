pub struct Pt {
    pub x: isize,
    pub y: isize,
}

impl Pt {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x: x, y: y }
    }
}

pub struct Board {
    v: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        let v = vec![vec![false; w]; h]; // off by default
        Self { v: v }
    }

    pub fn print(&self) {
        println!("{}", "-".repeat(self.v[0].len() + 2));
        for line in &self.v {
            print!("|");
            for &b in line {
                let mut c = " ";
                if b {
                    c = "X";
                }
                print!("{}", &c);
            }
            println!("|");
        }
    }

    pub fn set(&mut self, pt: Pt, b: bool) {
        self.v[pt.y as usize][pt.x as usize] = b;
    }

    // returns copy of the board iterated by one
    pub fn next(&mut self) -> Board {
        let mut b2 = Board::new(self.v[0].len(), self.v.len());
        for (y, row) in self.v.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let p = Pt::new(x as isize, y as isize);
                let n = self.neighbours(&p);
                // if alive -> stay alive with 2 or 3 neighbours
                // if dead -> come alive with exactly 3 neighbours
                if self.valid_and_alive(&p) {
                    if n == 2 || n == 3 {
                        b2.set(p, true);
                    }
                } else {
                    if n == 3 {
                        b2.set(p, true);
                    }
                }
            }
        }
        b2
    }

    pub fn neighbours(&self, pt: &Pt) -> usize {
        let mut count = 0;
        if self.valid_and_alive(&Pt::new(pt.x - 1, pt.y - 1)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x - 1, pt.y + 0)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x - 1, pt.y + 1)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x + 0, pt.y - 1)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x + 0, pt.y + 1)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x + 1, pt.y - 1)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x + 1, pt.y + 0)) {
            count += 1;
        }
        if self.valid_and_alive(&Pt::new(pt.x + 1, pt.y + 1)) {
            count += 1;
        }
        //println!("Cell ({},{}) has alive neighbours: {}", pt.x, pt.y, count);
        count
    }

    pub fn valid_and_alive(&self, pt: &Pt) -> bool {
        //println!("Checking Neighbour ({},{})", pt.x, pt.y);
        if self.valid_pt(pt) {
            //println!("Neighbour ({},{}) is valid", pt.x, pt.y);
            return self.v[pt.y as usize][pt.x as usize];
        }
        false
    }
    pub fn valid_pt(&self, pt: &Pt) -> bool {
        pt.x >= 0
            && pt.y >= 0
            && pt.x < (self.v[0].len() as isize)
            && pt.y < (self.v.len() as isize)
    }
    // pub fn get(&self, pt: Pt) -> bool {
    //     self.v[pt.y][pt.x]
    // }
    // pub fn height(&self) -> usize {
    //     self.v.len()
    // }
    // pub fn width(&self) -> usize {
    //     self.v[0].len()
    // }
}
