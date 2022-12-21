use std::fmt::Display;

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

    // this is valid b/c longest() and all is parameters and return types
    // share the same minimal scope
    let s3 = String::from("long string is long");
    {
        let s4 = String::from("xyz");
        let result = longest(s3.as_str(), s4.as_str());
        println!("The longest string is {}", result);
    }

    // this example does not work because s6's lifetime is smaller than
    // result's lifetime. So there is no guarantee result will still be valid
    // when it is used later on.
    // let s5 = String::from("long string is long");
    // let result;
    // {
    //     let s6 = String::from("xyz");
    //     result = longest(s5.as_str(), s6.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("lifetime struct: {}", i.part);

    // we also have access to the static lifetime, which indicates a variable can be
    // valid for the entire life of the program. But you should avoid using this lifetime
    // whenever possible. The compiler will often suggest it as a solution to compilation
    // issues, but it is often just a sympto of a different problem.
    let s: &'static str = "I have a static lifetime.";
}

// simple example of lifetimes, showing that the lifetime of the return type
// will be at least as long as both the x and y parameters. Effectively, for
// some lifetime 'a', all defined parameters must have at least lifetime of 'a'.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this also works fine b/c y cannot possibly be returned, we do not need to define any lifetimes
fn first_param<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this is not valid because we are trying to return a &str with no relationship to the
// defined lifetime.
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// we can annotate structs with lifetimes as well, like so. This means that any references
// in the struct must maintain the same lifetime as the instantiated object.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// we would also apply a lifetime to the impl
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// rust will infer the lifetime if it can, as in this example:
// Technically, all function have lifetime annotations, but rust can infer
// most cases. You only need to specify in ambiguous cases.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// This example shows using lifetimes with traits, genric types, and multiple parameters
// Brings it all together!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
