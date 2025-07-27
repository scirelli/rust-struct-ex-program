#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        Self{width:w, height:h}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
       self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 11,
    };
    let rect2 =  Rectangle {
        width: 1,
        height: 10,
    };
    let rect3 = Rectangle::new(5, 70);

    println!("{rect:?}\n{}\n{}\n{rect3:?}", rect.area(), rect.can_hold(&rect2));
}
