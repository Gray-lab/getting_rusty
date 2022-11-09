#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    dbg!(&rect1);
    println!("rect1 is: {:?}", rect1);

    // if there is a method with the same name as a field, rust knows the difference
    // because methods are followed by ()
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}