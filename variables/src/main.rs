fn main() {
    let mut a = 10;
    println!("mutable a: {a}");

    a = a + 5;
    println!("a + 5: {a}");

    let a = "Now I'm a string! This is called shadowing.";
    println!("{a}");

    {
        let a = "I can be redeclared here b/c of the closure.";
        println!("{a}");
    }

    const TWENTY_FOUR_HOURS_MS: u32 = 60 * 60 * 24 * 1000;
    println!("Milliseconds in 24 hours: {TWENTY_FOUR_HOURS_MS}");

    let hex = 0xff;
    let octal = 0x77;
    let bin = 0b1010_0101;
    println!("Numbers can be...");
    println!("Hex: {hex}");
    println!("Octal: {octal}");
    println!("Binary: {bin}");

    let float = 5.5;
    println!("Decimals use float: {float}");

    let addition = 50 + 50;
    println!("Addition: {addition}");

    let subtraction = 50 - 50;
    println!("Subtraction: {subtraction}");

    let multiplication = 50 * 50;
    println!("Multiplication: {multiplication}");

    let division = 50 / 50;
    println!("Division: {division}");

    let modulo = 50 % 2;
    println!("Modulus: {modulo}");

    let b = true;
    println!("Booleans too: {b}");

    let c = 'ɣ';
    println!("Char in rust is 4 bytes. It can do unicode as well as ASCII: {c}");

    let tup: (&str, &str, u8) = (
        "We can refer to tuples by index!",
        "We can refer to tuples by dereferencing:",
        50,
    );

    println!("Tuples!");
    let tup0 = tup.0;
    let (_, de_ref_tup1, de_ref_tup2) = tup;
    println!("{tup0}");
    println!("{de_ref_tup1} {de_ref_tup2}");

    println!("Arrays!");
    let arr: [u8; 4] = [1, 5, 10, 25];
    for x in arr {
        println!("{x}");
    }

    println!("Initialize an array with size and default value!");
    let arr2 = [100; 5];
    for x in arr2 {
        println!("{x}");
    }

    print_my_name_times("Matthew Stagg", 3);

    let exp = {
        let x = 1;
        x * 10 // adding a semicolon here makes it a statement, not an expression... so it will not return a value
    };
    println!("We can scope expressions: {exp}");

    let ten = ten();
    println!("Functions can exist as a single value expression: {ten}");
}

fn print_my_name_times(name: &str, times: u32) {
    for _ in 0..times {
        println!("My name is {name}.");
    }
}

fn ten() -> u8 {
    10
}
