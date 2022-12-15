mod median_and_mode;
mod pig_latin;

fn main() {
    let v = vec![1, 7, 7, 31, 7, 3, 12, 17, 26, 15, 3, 3, 33, 41];
    let v_median = median_and_mode::median(&v);
    let v_mode = median_and_mode::mode(&v);
    println!("v: {:?}", v);
    match v_median {
        Some(x) => println!("Median of v: {:?}", x),
        None => println!("Median of v: {:?}", "undefined"),
    }
    match v_mode {
        Some(x) => println!("Mode of v: {:?}", x),
        None => println!("Mode of v: {:?}", "undefined"),
    }

    let s = "Quick, hide the weed under the couch! I cannot go to jail.";
    let s_as_pl = pig_latin::convert(s);
    println!("English: {s}");
    println!("Pig Latin: {s_as_pl}");
}
