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

    let s1 = "My name is Matthew Stagg.";
    let s2 = "Quick, hide the weed under the couch! I cannot go to jail.";
    let s1_as_pl = pig_latin::from(s1);
    let s2_as_pl = pig_latin::from(s2);
    println!("English: {s1}");
    println!("Pig Latin: {s1_as_pl}");
    println!("English: {s2}");
    println!("Pig Latin: {s2_as_pl}");
}
