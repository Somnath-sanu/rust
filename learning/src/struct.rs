use std::fs::read_to_string;

struct User {
    active: bool,
    name: String,
    age: u32,
}

/**
    * function without &self in impl User will be a static func
    like in js, static func called directly on class, same
    here it will be called directly on User.
*/

impl User {
    fn get_age(&self) -> u32 {
        self.age
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn debug() -> bool {
        return true;
    }
}

// Enum

enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with values
enum Shape {
    Circle(f64), // floating 64 bit
    Rectangle(f64, f64),
}
fn main() {
    println!("Hello, world!");

    println!("{}", is_even(-255));
    println!("{}", fib(4));

    let my_string = String::from("Hello world!"); // "Hello world!" -> type is &str but the function expects String;

    let length = get_string_length(my_string);
    println!("The number of chars in the string is: {}", length);

    let user1 = User {
        active: true,
        name: String::from("shan"),
        age: 10,
    };

    println!("User name is: {}", user1.name);
    println!("User age is: {}", user1.get_age());
    println!("User active is: {}", user1.is_active());
    println!("User static func: {}", User::debug());

    let new_direction = Direction::East;
    let my_direction = new_direction; // Copy 

    let rect = Shape::Rectangle(4.0, 1.0);
    let area = calc_area(rect);
    println!("Area is {}", area);

    let index = find_first_a(String::from("preaet"));
    match index {
        Some(value) => println!("Index is {}", value),
        None => println!("a not found"),
    }

    let result = read_to_string("a.txt");
    match result {
        Ok(data) => println!("Data is: {}", data),
        Err(error) => println!("Error occured: {}", error),
    }
}

// Option enum (let u return some value or null value)
fn find_first_a(s: String) -> Option<i32> {
    for char in s.chars().enumerate() {
        if char.1 == 'a' {
            // char.0 -> index
            return Some(char.0 as i32);
        }
    }

    return None;
}

fn calc_area(shape: Shape) -> f64 {
    //* How can i get the value ? like rect[0] -> length , rect[1] -> breadth */
    // we can get that via pattern matching
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(radius) => 3.14 * radius * radius,
    };

    return area;
}

fn get_string_length(s: String) -> usize {
    s.chars().count() // implecitly return, we dont have to do return s... ;
}

fn is_even(num: i32) -> bool {
    //32 bit range => -2147483648 to 2147483647
    if num % 2 == 0 {
        return true;
    }
    return false;
}

// 0 1 1 2 3 5 8 13

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    for _i in 0..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}
