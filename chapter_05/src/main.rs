#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(20);

    println!("rect1: {:#?}\narea: {}", rect1, rect1.area());
    println!("rect2: {:#?}\narea: {}", rect2, rect2.area());

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1?: {}", rect2.can_hold(&rect1));
}
