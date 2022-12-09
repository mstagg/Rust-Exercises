fn main() {
    let rec = Rectangle {
        w: dbg!(50 * 10),
        h: 100,
    };

    let rec_small = Rectangle { w: 5, h: 5 };
    let rec_big = Rectangle { w: 500, h: 1000 };

    dbg!(&rec);
    println!("area: {}", rec.area());
    if (rec.width()) {
        println!("width is greater than 0");
    }
    if (rec.can_fit_rectangle(&rec_small)) {
        println!("Our rectangle can fit small rectangle");
    }
    if (!rec.can_fit_rectangle(&rec_big)) {
        println!("Our rectangle can not fit big rectangle");
    }

    // associated methods are accessed with the :: syntax
    let sq = Rectangle::square(10);
    println!("square x:{} y{}", sq.w, sq.h);
}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

// You can have multiple impl blocks for a struct, but the purpose is not
// explained until laer in the book
impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    // methods can have the same name as a property
    fn width(&self) -> bool {
        self.w > 0
    }

    fn can_fit_rectangle(&self, rec: &Rectangle) -> bool {
        self.w >= rec.w && self.h >= rec.h
    }

    // Associated functions are any function without self
    // They are most commonly used as constructors and are
    // equivalent to satic methods in other languages
    fn square(wh: u32) -> Self {
        Self { w: wh, h: wh }
    }
}
