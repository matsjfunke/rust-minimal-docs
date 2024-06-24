#[derive(Debug)] // Debug trait enables to print struct in a way we can see its value while we’re debugging 
struct Rectangle {
    width: u32,
    height: u32,
}

// method definition
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        self.width * 2 + self.height * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the rectangle dimesions {rect1:?}");
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The perimeter of the rectangle is {}", rect1.perimeter());
}
