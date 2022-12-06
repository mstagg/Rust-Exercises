fn main() {
    let str_stack = "Im a string on the stack, I cannot be mutated :("; // cannot be mutable, even if you use 'mut' keyword since the memory for it is allcoated at compile-time
    let mut str_heap = String::from("Im a string on the heap");
    str_heap.push_str(", I can be mutated :D");
    println!("{str_stack}");
    println!("{str_heap}");

    {
        let scoped_string = String::from("I am on the heap, with local scope.");
        println!("{scoped_string}");
    } // scoped_string is automatically freed when it leaves scope here

    // This creates 2 seperate primitives on the stack. x and y are seperate,
    // but both equal to 5.
    let x = 5;
    let y = x;

    // This creates 2 sets of primitives (as a String struct) that contain the same information.
    // Each String consists of a pointer, length, and capacity under the hood.
    // It copies the primitive values onto the stack, similar to the example above.
    // But each primitive collection refers to the same string in the heap!
    let s1 = String::from("test");
    let s2 = s1;

    // To prevent this, rust moves (not shallow copies!) the primitives in s1 into s2 and invalidates
    // s1 in the current scope. If you uncomment the line below, it will throw an error.
    // println!("{s1}");

    // If you want to explicitly copy the heap, there is a clone method:
    let s3 = s2.clone();

    // These same copy behaviors apply when passing variables into a function. They are
    // copied if theya re primitives and 'moved' if they are on the heap.
    makes_copy(y);
    takes_ownership(s3);

    // s3 was freed at the end of execution in the 'takes_ownership' function. It was moved
    // in and then freed when it went of of scope in the function. Calling it below would cause an error.
    // y can still be used because it was copied into the function, not moved.
    // println!("{s3}");
    println!("But you can still print y just fine: {y}");

    // You can return ownership by returning the struct from the function that originally took it.
    // But, it will need to be stored in a new var, unless you made it mutable explicitly.
    let s4 = String::from("test again");
    let s5 = takes_ownership_and_gives_it_back(s4);
    println!("My ownership was returned: {s5}");

    println!("This is all very tedious though. There should be a way to transfer ownership without all this pain.");
    println!("Well there is! Its called references!");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

fn takes_ownership_and_gives_it_back(some_string: String) -> String {
    // some_string comes into scope
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}
