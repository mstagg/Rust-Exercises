use std::{num, thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // we can also store closures as variables
    let my_closure = |n: u32| -> u32 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        n
    };

    my_closure(3);

    // Closures do not take owenership by default. If a closure modifies something,
    // the closure must also be marked as mut.
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before closure: {:?}", list);
    let mut closure = || list.push(10);
    // println!("In between closure: {:?}", list); // cannot use mutable closure before the mutation is finished
    closure();
    println!("After closure: {:?}", list);

    // You can force closures to take ownership as well with the `move` keyword. This is
    // used most commonly when using closures in threads, because threads need ownership.
    // The keyword tells rust to move any values into the closure
    let list2 = vec![10, 20, 30];
    println!("Before closure thread: {:?}", list2);
    thread::spawn(move || println!("closure from in thread: {:?}", list2))
        .join()
        .unwrap();

    let mut list3 = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    let mut num_sort_operations = 0;
    // let value = String::from("by key");

    // this doesnt work because we are trying to push values from inside the
    // closure to outside the closure. This violates ownership rules.
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // however, this works fine because we are not doing any move operations.
    list3.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list3);
    println!("num_sort_operations: {}", num_sort_operations);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
