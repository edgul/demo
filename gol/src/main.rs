use std::thread;
use std::time;

mod board;

use board::*;

fn main() {
    let mut b = Board::new(35, 20);
    b.set(Pt::new(0, 0), true);
    b.set(Pt::new(0, 1), true);
    b.set(Pt::new(0, 2), true);
    b.set(Pt::new(1, 1), true);

    // square
    b.set(Pt::new(20, 5), true);
    b.set(Pt::new(21, 5), true);
    b.set(Pt::new(20, 6), true);
    b.set(Pt::new(21, 6), true);

    // v-line
    b.set(Pt::new(21, 7), true);
    b.set(Pt::new(21, 8), true);
    b.set(Pt::new(21, 9), true);
    b.set(Pt::new(21, 10), true);

    let period = time::Duration::from_millis(1000);

    loop {
        b.print();
        b = b.next();
        thread::sleep(period);
    }
}
