pub mod hosting;
mod serving;

// you can reexport modules
// this both imports to this scope and exports
// you can also import multiple specific things
// self in this context also imports back_of_house as a namespace
pub use crate::back_of_house::{self, Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    println!("ABSOLUTE");
    crate::front_of_house::hosting::add_to_waitlist();
    println!("RELATIVE");
    hosting::add_to_waitlist();

    // order a breakfast with sourdough toast
    let mut meal = Breakfast::summer("sourdough");
    let side1 = Appetizer::Salad;
    let side2 = Appetizer::Soup;

    // wait we hate sourdough, order wheat
    meal.toast = String::from("wheat");

    // cant modify the seasonal_fruit field b/c its not marked with pub
    // meal.seasonal_fruit = String::from("blueberries");

    println!("I ate {} toast", meal.toast);
}
