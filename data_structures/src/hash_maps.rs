use std::collections::HashMap;

pub fn workspace() {
    println!("Lets learn about hash maps!");

    // types are also inferred if you provide values later in the same scope
    let mut score_table = HashMap::new();
    score_table.insert("Colts", 100);
    score_table.insert("Patriots", 0);

    // copied gives us a copy of the value instead of a reference to the value
    // unwrap gives the value if it exists, if it does not exists, it panics
    // unwrap_or_default does unwrap but defaults to the data type's default value
    // unwrap_or does unwrap but defaults to a given value if it is not defined
    println!(
        "Colts score is: {}",
        score_table.get("Colts").copied().unwrap()
    );
    println!(
        "Colts score is: {}",
        score_table.get("Patriots").copied().unwrap_or_default()
    );
    println!(
        "Broncos score is: {}",
        score_table.get("Broncos").copied().unwrap_or(1)
    );

    // we can iterate through a hashmap easily, however the order will be arbitrary
    for (key, val) in &score_table {
        println!("key: {} value: {}", key, val)
    }

    // primitives are copied, but complex objects are moved, as shown below
    let s1 = String::from("I lose owenership when put in a hashmap");
    let mut m1: HashMap<&str, &str> = HashMap::new();
    // this wont compile b/c we no longer have ownership
    // m1.insert("example", s1);
    // println!("{s1}");

    let mut m2 = HashMap::new();
    m2.insert("case1", 1);
    m2.insert("case2", 2);
    m2.insert("case3", 3);

    // if we insert at a key that already defined, we do an override
    m2.insert("case1", 10);
    println!("case1 is: {}", m2.get("case1").copied().unwrap());

    // there is an easy way to check if a value exists first.
    // this checks if a a key is defined, if it is not, it inserts 20
    // at that key
    m2.entry("case2").or_insert(20);
    println!("case2 is: {}", m2.get("case2").copied().unwrap());

    // we can also do inserts based on rpeviosu values, like so:
    let text = "How much wood could a woodchuck chuck if a woodchuck could chuck wood";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
