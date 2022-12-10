// when importing enums, structs, and items, import the whole path
use crate::garden::fruits::Orange;
// can rename imports with as
use crate::garden::vegetables::Asparagus as AAA;

mod new_mod_style;

// when importing functions, just import the scope right above it
use restaurant::back_of_house;
use restaurant::front_of_house::hosting::add_to_waitlist;

pub mod garden;

fn main() {
    let v = AAA {};
    let f = Orange {};
    println!("I'm growing {:?} and {:?}!", v, f);

    add_to_waitlist();
    restaurant::front_of_house::eat_at_restaurant();

    back_of_house::fix_incorrect_order();

    // this is the prefered way to handle modules though
    let a = new_mod_style::do_something();
    let b = new_mod_style::example::Things::A;
}
