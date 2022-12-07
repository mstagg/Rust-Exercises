fn main() {
    // these slice examples only work for ASCII strings
    // slicing examples for multi-byte characters are in a later section

    let s1 = String::from("Good Vibes Only");
    let good = &s1[..4]; // leaving the inclusive lower bound blank will start at the beginning of the string
    let vibes = &s1[5..10]; // the range is inclusive of the first index and exclusive of the second
    let only = &s1[11..]; //leaving the exlusive upper bound blank will include the rest of the string
    println!("{good} {vibes} {only}");

    let entire_string = &s1[..]; // this is a slice of the entire string
    println!("{entire_string}");

    let first_word = find_first_word(&s1, ' ');
    println!("{first_word}");

    let literal = "Literal String";
    let literal_first_word = find_first_word(literal, ' ');
    println!("{literal_first_word}");

    // string literals are slices as well, so our find_first_word function works on them
    // also. We can use the same function on both string literals and String objects.
    let s1_first_word = find_first_word(&s1, ' ');
    println!("{s1_first_word}");

    // There are also generic slices that can hold any primitive data type
    let a = [1, 2, 3, 4, 5];
    let slice = &a[3..];
    for val in slice.iter() {
        println!("{val}");
    }
}

fn find_first_word(s: &str, delimiter: char) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &c) in s_bytes.iter().enumerate() {
        if c == delimiter as u8 {
            return &s[..i];
        }
    }

    &s[..]
}
