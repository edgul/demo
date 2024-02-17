
// exclusive reference
fn print_mut_string(s: &mut String) {
    s.push('!');
    println!("{}", s);
}

// pass-by-value: move for String
fn print_imm_string(s: String) {
    println!("{}", s);
}

// shared reference
fn print_ref_string(s: &String) {
    println!("{}", s);
}

// pass-by-value, copy for int
fn print_num(i: u32) {
    println!("{}", i);
}

// way to copy?

fn main() {
    let immutable_s = String::from("Immutable string");

    // in order to pass to function we "cast" to exclusive reference 
    // original variable must be declared as mut
    let mut mutable_s = String::from("Mutable string");
    
    //let mut_ref_s = &mut mutable_s;         // local mut ref is not fine here 
                                              // as below borrows would fail
    //let shared_ref = &mutable_s;            // same with shared ref
                                              // we wouldn't be able to get exlusive lock in next call
    
    // pass by reference by getting mutable reference
    // print_mut_string(mutable_s);         // Error: mismatched types, attempting to pass String 
    // print_mut_string(&mutable_s);        // Error: mismatched types, attempting to pass &String 
    print_mut_string(&mut mutable_s);       // prints Mutable string!
    // print_mut_string(&mut immutable_s);  // Error: cannot mutate immutable variable
    print_mut_string(&mut mutable_s);       // prints Mutable string!!
    
    let mut_ref_s = &mut mutable_s;         // local mut ref is fine here
    print_mut_string(mut_ref_s);            // prints Mutable string!!!
    
    print_ref_string(&immutable_s);         // Not moved, shared ref
    print_imm_string(immutable_s);          // Value moved
    // print_imm_string(immutable_s);       // Error: use after move

    // integers are slightly different
    let i = 4;
    // i += 1;      // Error: same mutability rules
    print_num(i);   // but, pass-by-value does not consume for int
    print_num(i);


    // println!("Hello, world!");
}
