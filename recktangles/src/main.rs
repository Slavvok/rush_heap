#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
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

//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

fn main() {
    //let scale = 2;
    //let rect1 = Rectangle {
    //    width: dbg!(30 * scale),
    //    height: 50,
    //};
    //dbg!(&rect1);
    
    //println!("rect1 is {:?}", rect1);
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area is {}",
        rect1.area()
    );

    // Check if can hold another Rectangle

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Check if rect1 can hold rect2: {}", rect1.can_hold(&rect2)); 
}
