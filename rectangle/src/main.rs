fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        height: 45,
        width: 60,
    };
    let rect4 = Rectangle::square(40);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("{:#?}", rect4);
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}