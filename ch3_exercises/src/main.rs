use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 0.559
}

fn fib(n: u32) -> u128 {
    fn fib_recursive(n: u32, prev_val: u128, current_val: u128) -> u128 {
        match n {
            0 => prev_val,
            1 => current_val,
            _ => fib_recursive(n - 1, current_val, prev_val + current_val),
        }
    }

    fib_recursive(n, 0, 1)
}

fn main() {
    let mut temp_input_raw = String::new();

    println!("Input a F temperature: ");
    io::stdin()
        .read_line(&mut temp_input_raw)
        .expect("Failed to read temperature");
    let temp: f64 = temp_input_raw
        .trim()
        .parse()
        .expect("Failed to parse temperate input");
    let celsius = fahrenheit_to_celsius(temp);

    println!("{:.2}F is {:.2}C", temp, celsius);

    println!("How many fibonacci positions to calculate: ");
    let mut fib_input_raw = String::new();
    io::stdin()
        .read_line(&mut fib_input_raw)
        .expect("Failed to read fibonacci input");
    let f: u32 = fib_input_raw
        .trim()
        .parse()
        .expect("Failed to parse fibonacci input");
    let fibonacci = fib(f);

    println!("Fibonacci number {f} is {fibonacci}");

    println!("Im not printing out 'Merry Christmas'. I know how loops work.");
}
