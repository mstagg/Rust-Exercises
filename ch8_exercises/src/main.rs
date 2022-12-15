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
    let s2 = "I work at a company called 30rock.";
    let s3 = "Please, hide the weed under the couch! I will lose my job.";
    let s4 = "I would be in jail for 25 years!";
    let s1_as_pl = pig_latin::from(s1);
    let s2_as_pl = pig_latin::from(s2);
    let s3_as_pl = pig_latin::from(s3);
    let s4_as_pl = pig_latin::from(s4);
    println!("English: {s1}");
    println!("English: {s2}");
    println!("English: {s3}");
    println!("English: {s4}");
    println!("Pig Latin: {s1_as_pl}");
    println!("Pig Latin: {s2_as_pl}");
    println!("Pig Latin: {s3_as_pl}");
    println!("Pig Latin: {s4_as_pl}");
}
