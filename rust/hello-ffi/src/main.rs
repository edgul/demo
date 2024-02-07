
extern "C" {
    fn hello_from_c(); 
}

fn safe_hello() {
    unsafe { hello_from_c() }
}
fn main() {
    safe_hello();
}
