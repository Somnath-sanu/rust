// All variables in rust are by default immutable
fn main() {
    let a = 32;
    let b = -22;
    let result = add(a, b);
    println!("Result is {}", result);
    println!("{}", a); // store in stack not in heap
    let mut str = String::from("Hello");
    str = print_str(str); // str owns the value again
    println!("{}", str); // now no ownership error

    // without making str mutable and re-assigning
    let mut str = String::from("Hello");
    print_str_1(&mut str); // passing the reference //*Borrowing */
    println!("Original String : {}", str); // now again no ownership error
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn print_str(s: String) -> String {
    println!("{}", s); // now s owns the value
    return s;
}

fn print_str_1(s: &mut String) {
    s.push_str("sir");
    println!("{}", s); // now s owns the value
}
