pub fn workspace() {
    println!("Lets learn about strings!");

    // most vector methods are also available on strings, b/c
    // strings are just wrappers around vector.

    // new empty String
    let mut s = String::new();

    // We can transform and load literals into s like so...
    // Any data wih the Display trait will ahve the to_string method
    s = "Here is some text.".to_string();
    println!("{s}");

    // strings are utf-8 encoded. Meaning they ccan contain more than the
    // standard ASCII character set
    let hello = String::from("Is utf-8 by default: السلام عليكم");
    println!("{hello}");

    // we can grow strings in several different ways
    s.push_str(" We can push strings onto existing strings.");
    println!("{s}");
    s.push('K');
    println!("{s}");

    // we can append strings with +, but the second argument must be a string slice.
    // This is because + is shorthand for the .add function that has a signature of
    // .add(self, s: &str) -> String
    let s2 = String::from("We can add ");
    let s3 = String::from("strings together with +.");
    let s4 = s2 + &s3;
    println!("{s4}");
    println!(
        "{}",
        String::from("We") + " can" + " chain" + " together" + " multiple" + " additions."
    );
    println!(
        "{}",
        format!("{}-{}-{}-{}-{}", "Or", "we", "can", "use", "format!()")
    );

    // push_str accepts a string slice, so it doesnt take ownership. We can use the
    // slice after it is appended.
    let s5 = "FOO";
    s.push_str(s5);
    println!("{s}");
    println!("{s5}");

    // Cannot access string contents by index in rust. This is because strings are utf-8.
    // Utf-8 strings can be multiple bytes, so indexing may return a byte that is just a
    // part of a character.
    // let s = s5[0];

    // We can, however, get string slices with the following syntax. This is dangerous,
    // however. Not all string characters are a single byte! See second example.
    let s6 = &s5[..1];
    println!("First character of s5: {s6}");

    let s7 = "Здравствуйте";
    let s8 = &s7[..2];
    // let s8 = &s7[..2]; // This will panic b/c it will index 1.5 characters
    println!("First character of s7 is two bytes: {s8}");

    // It is safer to iterate over a string with .chars() or bytes() and work with
    // the resulting characters. chars() gives char types and bytes() gives the raw
    // byte values.
    for c in s7.chars() {
        println!("{c}");
    }
}
