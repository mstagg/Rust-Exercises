use std::error;
use std::fs::File;
use std::io::{self, Read};

// We can also return Results from main. It doesnt have to be void.
fn main() -> Result<(), Box<dyn error::Error>> {
    // Normally, programs will clean up the stack before they exit. This is default
    // behavior. However, if you want to exit without cleaning up the stack, you can
    // add panic = 'abort' to the cargo.toml file. This will cause the program to abort
    // without cleaning up the stack. This makes the executable as small as possible,
    // but the OS will need to clean up the memory afte program execution.

    // Simplest way to throw an error
    // panic!("Houston, we have a problem...");

    // Your code can also panic from ilelgal behavior, in this example, out of bounds.
    // By setting the RUST_BACKTRACE=1 environment variable, you can see the error stack.
    // But the stack will only be printed in debug mode, unless you specifically set
    // debug symbols to be enabled in the relase build.
    // let my_vec = vec![1, 2, 3];
    // let out_of_bounds = my_vec[99];

    // Use match to handle errors in a more controlled way. You can also do this with
    // a closure, which is covered later, to remoe some boilerplate.
    let file_name = "hello.txt";
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match File::create(file_name) {
                Ok(nf) => nf,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            any_other_error => panic!("Problem opening the file: {:?}", any_other_error),
        },
    };

    // You can use unwrap as a shortcut to panic!(). If the result is Ok, it will return
    // the contents of the Ok. If the result is Err, it will panic!() with the error.
    // let error_file = File::open("I_DONT_EXIST.txt").unwrap();

    // You should use expect() isntead of unwrap() as a best practice because it allows
    // you to specify a custom error message. This is useful for debugging.
    // let error_file = File::open("I_ALSO_DONT_EXIST.txt").expect("This file never exists.");

    // You can resturn errors. Propagating them up to the calling function
    // let value = read_from_file().expect("This error is propagated with '?'.");

    // You cannot use '?' in main, that returns void. You can use it if main returns a Result.
    // let fake_file = File::open("fake.txt")?;

    let last_char = last_char_of_first_line("").expect("No last char found.");

    Ok(())
}

// '?' is shorthand to return an error without writing out a match statement.
fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("value.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// '?' can also be used with Option. Use it to return None early.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
