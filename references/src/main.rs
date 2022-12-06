fn main() {
    // By passing in a pointer as a reference to the string,
    // we cause the refernece to go out of scope. NOT the original string.
    // This allows us to use the string in the original scope b/c
    // we never moved owenership of it.
    let s1 = String::from("THIS_IS_BORROWING");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");

    // Passing in a mutable reference allows it to be changed.
    let mut s2 = String::from("MUTABLE_STRING");
    attempt_to_change(&mut s2); // <-- notice reference AND value are mutable
    println!("This string was modifed: {s2}");

    // Code below will fail to compile. This is because a value can
    // only have a single mutable reference to it at any given time. This prevents
    // race conditions by guranteeing a value can only ever be owned and modified
    // by a single scope at any given time.
    // let mut s3 = String::from("MULTIPLE_REFERENCES");
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // println!("This will fail: {r1} {r2}");

    // This is will also fail to compile. You may have as many immutable
    // references to a value as you want. But the moment you introduce a
    // mutable reference, you can only have a single reference.
    // let mut s3 = String::from("MULTIPLE_REFERENCES");
    // let r1 = &s3;
    // let r2 = &s3;
    // let r3 = &mut s3;
    // println!("This will fail: {r1} {r2} {r3}");

    // However, references go out of scope once the program has reached a point
    // in execution where they are no longer used. You may apply a mutable
    // reference when there are multiple immutable references IF those immutable
    // references are no longer used.
    let mut s3 = String::from("MULTIPLE_REFERENCES");
    let r1 = &s3;
    let r2 = &s3;
    println!("This will compile: {r1} {r2}");
    // r1 and r2 go out of scope here ^^^

    let r3 = &mut s3;
    println!("This will also compile: {r3}");

    let s4 = dangle_me_timbers();
    println!("Rust prevents dangling references: {s4}");
}

// Rust refers to this act of passing a pointer as 'borrowing'
fn calculate_length(s: &String) -> usize {
    s.len()
}

// The below funtion will not work b/c rust does not allow you
// to modify a borrowed value by default. If you borrow something,
// it is read-only.
// fn attempt_to_change(s: &String) {
//     s.push_str("foo");
// }
//

fn attempt_to_change(s: &mut String) {
    s.push_str("(foo)");
}

// Below will not compile b/c it will create a dangling reference. You
// cannot return a reference to a value that has has gone out of scope,
// rust will not let you. This prevents the dangling pointer problem.
// fn dangle_me_timbers() -> &String {
//     let s = String::from("foo");
//     &s
// } // s goes out of scope here and is dropped from memory, but we still try to return a reference to it, this causes an error

fn dangle_me_timbers() -> String {
    let s = String::from("foo");
    s
}
