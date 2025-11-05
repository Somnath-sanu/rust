// traits
trait Summary {
    fn summary(&self) -> String;
}

struct User {
    name: String,
}

impl Summary for User {
    fn summary(&self) -> String {
        return format!("User name is {} and summary is haha", self.name);
    }
}

fn notify<T: Summary>(item: &T) {
  println!("Breaking news {}", item.summary())
}

fn main() {
    let user = User {
        name: String::from("Rohan"),
    };
    notify(&user);
    println!("{}", user.summary());
}
