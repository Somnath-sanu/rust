use std::ops::{Add, Mul};

struct Rect<T> {
    width: T,
    height: T,
}
enum Role<T> {
    Admin(T),
    User
}


impl<T: Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        self.height * self.width
    }
}


fn main() {
    let r1 = Rect {
        width: 10,
        height: 10,
    };

    let r2 = Rect {
        width: 10.1,
        height: 10.1,
    };

    println!("Area of rec is {}", r1.area());
    println!("Area of rec is {}", r2.area());

    println!("{}", sum(1, 2));

    let status = Role::Admin(sum(10,10));
    match status {
        Role::Admin(val) => println!("{}",val),
        Role::User => println!("User")
    }
}

// Add<Output = T> -> Trait Bound
fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}
