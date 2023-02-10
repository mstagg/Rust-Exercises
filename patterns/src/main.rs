fn main() {
    // simple patterns to match by type
    // in this type of pattern matching, the match must be exhaustive
    let x1 = Some(25);
    match x1 {
        None => println!("x1 is empty"),
        Some(val) => println!("x1 contains {val}"),
    }

    // if lets can be chained to do more advanced pattern matching
    // however, the compiler wont enforce exhaustiveness
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // we can combine if let with a loop to continously do an action
    // based on the pattern match result

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Found a value: {}", top);
    }

    // pattern matching can also destructure, like in this for loop
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // technically, every let is the same pattern as if let
    // let PATTERN = EXPRESSION
    // this destructure is valid
    let (x, y, z) = (1, 2, 3);

    // however, you must destructure each arg
    // this is invalid
    // let (x, y) = (1, 2, 3);

    // function args can destructure as well
    let point = (3, 5);
    print_coordinates(&point);

    // we can also match literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // using OR, we can match multiple patterns in a single expression
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // we can also match ranges
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // this also works on characters, but not strings
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // We can destructure struct properties similar to javascript
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // we can even destructure and match against values from a struct in the same expression
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // we can easily match against enums as well
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // we can combine all of these into truly wild destructing patterns
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(1, 2);

    // we can also ignore values in match statements
    let x = Some(100);
    match x {
        Some(_) => println!("x contains something"),
        None => println!("x is empty"),
    }

    // we can macth multiple unrelated values as well
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // _ can ignore specific destructured values as well
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // rust wont complain about unused vars with _ prefix
    let _a = 5;

    // .. can be used in place of _ to ignore multiple propeties at once
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // can do this with tuples too
    // this works b/c tuples are ordered lists
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // we can also match conditionals using match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // match guards can extract specific values from multiple combined expressions
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // can use @ to assign a var from a range to a var that can operated on as well
    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

// function args can destructure as well
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// we can ignore arguments if we want and the compiler wont complain by using _
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message2 {
    Hello { id: i32 },
}
