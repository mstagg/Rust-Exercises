use rand::Rng;

const LOOP_MAX: u8 = 3;

fn main() {
    let num = rand::thread_rng().gen_range(0..=10);
    println!("Random number is {num}");

    let num_is_even = num % 2 == 0;
    if num_is_even {
        println!("{num} is even.")
    } else {
        println!("{num} is odd.")
    }

    // no ternary, instead use if in-line
    let inline = if num_is_even { "EVEN" } else { "ODD" };
    println!("Inline: {inline}");

    let mut counter = 0;
    loop {
        println!("loop: {}", counter + 1);
        counter += 1;
        if counter >= LOOP_MAX {
            break;
        }
    }

    println!("Get value from a loop.");
    let mut counter = 0;
    let loop_val = loop {
        if counter != LOOP_MAX {
            println!("{counter} is less than {LOOP_MAX}");
            counter += 1;
        } else {
            break counter;
        }
    };
    println!("Got {loop_val} from loop.");

    println!("You can label loops and refer to them.");
    let mut outer_counter = 0;
    'outer_loop: loop {
        let mut inner_counter = 0;
        println!("outer counter: {}", outer_counter + 1);

        'inner_loop: loop {
            println!("inner counter: {}", inner_counter + 1);
            if inner_counter >= LOOP_MAX {
                break 'inner_loop;
            }
            if outer_counter >= LOOP_MAX {
                break 'outer_loop;
            }
            inner_counter += 1;
        }

        outer_counter += 1;
    }

    let mut counter = 0;
    while counter < LOOP_MAX {
        println!("while: {}", counter + 1);
        counter += 1;
    }

    let collection = [1, 2, 3];
    for item in collection {
        println!("for x in y: {item}")
    }

    for i in 0..LOOP_MAX {
        println!("for: {i}");
    }

    for i in (0..LOOP_MAX).rev() {
        println!("for reverse: {i}");
    }
}
