// you can import everything in a package with the * operator
use crate::front_of_house::*;

pub fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {
    println!("cooking order");
}

mod scope_example {
    // use only works in the scope it is declared in,
    // if you moved this outside the scope, to the top of the file,
    // if would throw an error
    // comment line below to see
    use crate::front_of_house::hosting;

    fn example() {
        hosting::add_to_waitlist()
    }
}

// for structs, muct mark the top level struct AND each property with pub
// to expose it to parent modules
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// the impl does NOT need to be marked pub, but the member functions do
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// on the other hand, marking an enum public makes everything in it public
pub enum Appetizer {
    Soup,
    Salad,
}
