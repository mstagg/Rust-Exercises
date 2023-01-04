fn main() {
    // iterators are lazy, meaning they arent created and evaluated until it is necesary to do so.
    let v1 = vec![1, 2, 3];
    let v1_as_iter = v1.iter();

    // we can loop through an iter like this... in fact, when you lopp through an array or
    // vector, you are converting to an interator implicitly
    for x in v1_as_iter {
        println!("For loop: {x}");
    }

    // Iterators are a trait, so you can work any data struture into a iterator and use
    // it with the standard lib if you find the need

    // looping through an iterator uses the next method. Calling this method changes the
    // internal state, which requires making the iterator mut and understanding you cannot
    // "undo" a next method call as it 'pops' out the value when its called.
    let v2 = vec![4, 5, 6];
    let mut v2_as_iter = v2.iter();
    println!("Next call 1: {}", v2_as_iter.next().expect("not found"));
    println!("Next call 2: {}", v2_as_iter.next().expect("not found"));
    println!("Next call 3: {}", v2_as_iter.next().expect("not found"));

    // Iterables are immutable by defualt. We can also use into_iter() to create a iterator
    // that assumes ownership of its collection. iter_mut() can be called to get mutable
    // references.

    // You can chain iterators with consumers to create new values
    let v3 = vec![7, 8, 9];
    let sum_of_v3: i32 = v3.iter().sum();
    println!("Summed iterator: {sum_of_v3}");

    // Some iterator consumers will return a new iterator, which can then be chained into
    // more itnerators, and so on...
    let v4 = vec![10, 11, 12];
    let sum_of_v4_squares: i32 = v4.iter().map(|x| x * x).sum();
    println!("Summed squares: {sum_of_v4_squares}");

    // We can also collect iterators into a new type of collection
    let v5 = vec![13, 14, 15];
    let v5_as_halves: Vec<f64> = v5.iter().map(|y| *y as f64 * 0.5).collect();
    println!("Iterator halved: {:?}", v5_as_halves);

    // Some iterable consumers must resolve to a bool, like filter
    let v6 = vec![16, 17, 18];
    let v6_as_evens: Vec<i32> = v6.into_iter().filter(|z| z % 2 == 0).collect();
    println!("Iterator evens: {:?}", v6_as_evens);
    // println!("Lost ownership: {:?}", v6); // This no longer compiles because we took ownership with into_iter above
}
