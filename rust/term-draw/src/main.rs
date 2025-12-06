/* term-draw
 *
 * A small program that can draw command line banners.
 * A user can walk through banners via text entry.
 *
 */

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::cell::Cell;

fn draw_line_h(len: u8, inner_c: char, border_c: char, text: &str) {
    assert!(len > 2);
    assert!(usize::from(len) > text.len());
    print!("{}", border_c);

    let non_text_width = usize::from(len-2) - text.len();
    let non_text_before = non_text_width/2;
    let non_text_after = non_text_width - non_text_before;

    for _ in 0..non_text_before {
        print!("{}", inner_c);
    }
    print!("{}", text);
    for _ in 0..non_text_after {
        print!("{}", inner_c);
    }
    println!("{}", border_c);
}

fn draw_window_inner(width: u8, height: u8, text: &str) {
    assert!(width > 2);
    assert!(height > 2);
    draw_line_h(width, '*', '*', "");
    for i in 0..height-2 {
        if i == (height-2)/2 {
            draw_line_h(width, ' ', '*', text);
        } else {
            draw_line_h(width, ' ', '*', "");
        }
    }
    draw_line_h(width, '*', '*', "");
}

fn draw_window(window: &Window) {
    draw_window_inner(window.width, window.height, &window.text);
}

fn get_user_text() -> String {
    let mut s = String::new();
    print!("$: ");
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s
}

fn clear_screen() {
    for _ in 0..75 {
        println!("");
    }
}

struct Window<'a> {
    text: String,
    width: u8,
    height: u8,
    next: Cell<Option<&'a Window<'a>>>,
    back: Cell<Option<&'a Window<'a>>>,
}

impl<'a> Window<'a> {
    fn new(text: &str, next: Option<&'a Window<'a>>) -> Self {
        Self {
            text: text.to_string(),
            width: 25,
            height: 5,
            next: Cell::new(next),
            back: Cell::new(None), 
        }
    }
    fn set_back(&self, back: &'a Window<'a>) {
        self.back.set(Some(back));
    }
}

fn main() {
    let mut inner = Window::new("last", None);
    let outer = Window::new("hello", Some(&inner));
    inner.set_back(&outer);

    let mut window = Some(&outer);
    let mut warning = "";
    let mut exit = false;
    loop {
        clear_screen();
        draw_window(window.unwrap());
        if !warning.is_empty() {
            println!("{}", warning);
            warning = "";
            if exit {
                break;
            }
        }
        let raw_s = get_user_text();
        let s = raw_s.trim();
        //println!("{}", s);
        if s == "quit" {
            warning = "quitting";
            exit = true;
            continue;
        } else if s == "next" {
            if window.unwrap().next.get().is_none() {
                warning = "DONE";
                exit = true;
                continue;
            }
            window = window.unwrap().next.get();
        } else if s == "back" {
            if window.unwrap().back.get().is_none() {
                warning = "You can't go back";
                continue;
            }
            window = window.unwrap().back.get();
        }
    }
}

fn _test_basic_window() {
    draw_window_inner(20,3, "");
    draw_window_inner(20,3, "hello");
    draw_window_inner(20,3, "hello1");
    draw_window_inner(25,3, "");
    draw_window_inner(25,3, "hello");
    draw_window_inner(25,3, "hello1");
    draw_window_inner(25,4, "");
    draw_window_inner(25,4, "hello");
    draw_window_inner(25,4, "hello1");
    draw_window_inner(25,5, "");
    draw_window_inner(25,5, "hello");
    draw_window_inner(25,5, "hello1");
}
