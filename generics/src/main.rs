fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['n', 'y', 'c', 'a', '7', 'B', 'A', 'e'];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer_point = Point { x: -2, y: 15 };
    let float_point = Point { x: 1.5, y: 7.0 };
    println!("integer point: {:?}", integer_point);
    println!("float point: {:?}", float_point);

    // This wont compile because the Point only accepts a single type argument, but
    // we provide two different types.
    // let mixed_point = Point { x: 2.5, y: 5 };

    // However, you can provide multiple type arguments
    let mixed_point = Mixed_Point { x: 2.5, y: 5 };
    println!("mixed point: {:?}", mixed_point);

    println!("generic typed method: {:?}", integer_point.x());
    println!("specific typed method: {:?}", float_point.y());
    // this wont compile because y() is not implemented for generic Point
    // println!("specific typed method: {:?}", integer_point.y());

    let smashup_point = float_point.mixup(mixed_point);
    println!("smashup point: {:?}", smashup_point);
}

// This wont compile unless we restrict the type of T to values that can be compared...
// in this case, types that implement the PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// can also use generic types on structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// can also use generic types on methods in implementations
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // You are not restricted to just the generic types declared on the struct or impl.
    // You can use any addituonl generic types on methds.
    fn mixup<NewX, NewY>(self, other: Mixed_Point<NewX, NewY>) -> Mixed_Point<T, NewY> {
        Mixed_Point {
            x: self.x,
            y: other.y,
        }
    }
}

// can also use add methods only for specific types!
impl Point<f64> {
    fn y(&self) -> f64 {
        &self.y * 10.0
    }
}

// can use multiple generic types on structs
#[derive(Debug)]
struct Mixed_Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Mixed_Point<T, U> {}
