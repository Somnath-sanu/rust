
// if u Implement copy trait , u have to implement clone traint as well
#[derive(Debug, Clone, Copy)] // no such micro for Display trait
struct User {
    //username: String, // copy trait cannot be implemented for string
    age: u32,
}
fn main() {
    let u = User {
       // username: String::from("Shanu"),
        age: 32,
    };

    let bb: String = "yoo".into();
    println!("{}",bb);

    let v = u;

    println!("{:?} , {:?}", u , v); // debug trait
}
