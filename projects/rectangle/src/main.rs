#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_contain(&self, rect: Rectangle) -> bool {
        if self.area() >= rect.area() {
            return true;
        }
        false
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 100,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 5,
    };

    let can_hold = rect1.can_contain(rect2);

    println!("rect1 can hold rect2? {can_hold}");
}
