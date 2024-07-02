#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rectangle: {:?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );

    // let scale = 2;
    // let rect = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };
    //
    // dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    if rectangle.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle.width);
    }

    let r1 = Rectangle {
        width: 30,
        height: 40,
    };
    let r2 = Rectangle {
        width: 20,
        height: 10,
    };

    println!("Can r1 hold r2: {}", r1.can_hold(&r2));

    let square = Rectangle::square(4);
    println!("square: {:?}", square);

}
