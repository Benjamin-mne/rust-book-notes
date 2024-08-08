#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        if self.area() > other_rect.area() {true} else {false}
    }

    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let sq = Rectangle::square(3);

    println!("Rect1 can hold the square is: {}", rect1.can_hold(&sq));

    // Function vesrion
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Method vesion
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "rect1 is {rect1:#?}"
    );

    dbg!(&rect1);
}

// Function version: without implementation way
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}