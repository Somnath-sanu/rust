use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

fn find_area<T: Shape>(s: &T) {
    println!("From Main func {}", s.area());
}

// OR 

fn find_area_2(s: &impl Shape) {
    println!("From Main func 2 {}", s.area());
}

// OR
fn find_area_3<T>(s: &T) where T: Shape {
    println!("From Main func 3 {}", s.area());
}

fn main() {
    let c = Circle { radius: 10.1 };
    println!("Area of circle is {}", c.area());
    find_area(&c); // so that I didn't move the ownership as I have to use it again
    find_area_2(&c);
    find_area_3(&c);
}
