fn main() {
    let ipv4 = IpAddrType::V4(127, 0, 0, 1);
    let ipv6 = IpAddrType::V6(String::from("::1"));

    route(ipv4);
    route(ipv6);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 5, y: 5 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 255, 255);
    m1.call();
    m2.call();
    m3.call();
    m4.call();

    let x = Some(10);
    let y: Option<i32> = None;
    let z = 100;

    // the compiler guarantees all match cases are handled
    match x {
        Some(n) => println!("defined: {n}"),
        None => println!("undefined"),
    }
    match y {
        Some(n) => println!("defined: {n}"),
        None => println!("undefined"),
    }

    // this wont compile b/c there is a chance the Option type is
    // undeefined
    // let zzz = x + z;
    let zzz = match x {
        Some(n) => n + z,
        None => panic!("x is undefined"),
    };
    println!("This compiles fine: {zzz}");

    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Penny: {}", value_in_cents(Coin::Nickel));
    println!("Penny: {}", value_in_cents(Coin::Dime));
    println!("Penny: {}", value_in_cents(Coin::Quarter(State::Alaska)));

    let a = match increment(Some(5)) {
        Some(x) => x,
        None => 0,
    };
    let b = match increment(None) {
        Some(x) => x,
        None => 0,
    };
    println!("increment some: {a}");
    println!("increment some: {b}");

    // you can have wilcard, catch-all match results
    let num = 7;
    match num {
        1 => println!("its 1"),
        2 => println!("its 2"),
        x => println!("its not 1 or 2, its {x}"),
    }
    // or you can ignore individual args
    let num = 7;
    match num {
        1 => println!("its 1"),
        2 => println!("its 2"),
        _ => println!("its not 1 or 2, its something else"),
    }

    let some_value = Some(3);
    // we could handle the value this way
    // match some_value {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // But this syntax is shorter and easier. Its great if you need to write
    // a match statement that only handles a single specific case with a wildcard for all
    // other cases.
    if let Some(max) = some_value {
        println!("The value is configured to be {}", max);
    }

    // if let also supports else statements
    let mut count = 0;
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {count}");
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => {
            println!("Lucky Coin!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("Quarter from {:?}", s);
            25
        }
    }
}

enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: IpAddrType) {
    match ip {
        IpAddrType::V4(..) => println!("This is V4"),
        IpAddrType::V6(..) => println!("This is V6"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => println!("Message"),
            Message::Move { .. } => println!("Move"),
            Message::Write(..) => println!("Write"),
            Message::ChangeColor(..) => println!("ChangeColor"),
        }
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
