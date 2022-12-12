pub fn workspace() {
    println!("Lets learn about vectors!");

    // new, empty vectors require a type annotation b/c the compiler cannot infer what
    // typw will be in the vector
    let v1: Vec<i32> = Vec::new();

    // rust also provides a macro for initializing a new vector with values.
    // it will infer the type
    let v2 = vec![1, 2, 3, 4, 5];

    // adding values to the end of the vector is easy, use .push
    // the type is infered b/c we push in values after initialization.
    // we have to mark it as mutable to add new values.
    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);

    // you can access vector members with .get or by index
    // .get returns an option and needs to be handled, which makes it safer
    // in regads to out of bounds exceptions
    let by_index = &v3[0];
    let by_get = match v3.get(0) {
        Some(x) => x,
        None => panic!("Not Found!"),
    };

    // Below code will not compile b/c we are mutating something while
    // it is borrowed (by_index and by_get). It works fine if its applied
    // after the borrow is complete.
    // v3.push(40);

    println!("We can get by index: {by_index}");
    println!("Or we can use .get: {by_get}");

    // we can easily loop through everything in a vector
    println!("Vectors are easy to loop through");
    loop_and_display(&v3);

    // we can also loop through a vecgor and change the values in place
    println!("Vectors can also be modified in place");
    loop_and_square(&mut v3);
    loop_and_display(&v3);

    // you can have a vector contain more than 1 type if you provide them all in
    // an enum so ti knows what to expect.
    let v4 = vec![
        Different_Types::Int(100),
        Different_Types::Float(100.0),
        Different_Types::Text(String::from("100")),
    ];
    println!("This vector consists of different types");
    loop_and_display_multiple_types(&v4);

    // Vec also has a useful .pop method that can be used like a stack
    println!("We can also use a vector like a stack:");
    let mut v5: Vec<i32> = Vec::new();
    v5.push(1);
    v5.push(2);
    v5.push(3);
    while let Some(x) = v5.pop() {
        println!("{x}");
    }
}

enum Different_Types {
    Int(i32),
    Float(f64),
    Text(String),
}

fn loop_and_display(vector: &Vec<i32>) {
    for x in vector {
        println!("{x}");
    }
}

fn loop_and_display_multiple_types(vector: &Vec<Different_Types>) {
    for x in vector {
        match x {
            Different_Types::Int(x) => println!("This is an integer: {x}"),
            Different_Types::Float(x) => println!("This is a float: {x}"),
            Different_Types::Text(x) => println!("This is a string: {x}"),
        }
    }
}

fn loop_and_square(vector: &mut Vec<i32>) {
    for x in vector {
        // * is a dereference and it refers to the value at the memory
        // location that the reference points to.
        *x *= *x
    }
}
