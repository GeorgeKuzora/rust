#[derive(Debug)]
struct  Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.width
    }

    fn square(size: u32) -> Self {
        Self { height: size, width: size }
    }
}

fn main() {
    let height = 30;
    let width = 50;

    let scale = 2;

    let rectangle = Rectangle {
        height,
        width: dbg!(width * scale)
    };

    dbg!(&rectangle);
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    println!("rect1 is {rectangle:#?}");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
