fn main() {
    // structs are similar to classes in other languages...
    // or objects in JavaScript
    let house = House {
        color: String::from("yellow"),
        bedrooms: 2,
        bathrooms: 1,
        price_in_cents: 120_000 * 100,
    };
    println!("This house costs ${:.2}", { house.price_in_cents / 100 });

    // making a struct mutable makes all of its properties mutable
    // you cannot mark specific fields as mutable.
    let mut mutable_house = House {
        color: String::from("red"),
        bedrooms: 3,
        bathrooms: 2,
        price_in_cents: 190_000 * 100,
    };
    println!("This house was {}", mutable_house.color);
    mutable_house.color = String::from("purple");
    println!("But we repainted it {}", mutable_house.color);

    let built_house = build_house(String::from("orange"), 4, 2);
    println!(
        "I built this house with a function. It is a {} {}bed/{}bath worth ${:.2}.",
        built_house.color,
        built_house.bedrooms,
        built_house.bathrooms,
        built_house.price_in_cents / 100
    );

    // You can spread properties, but explicitly defined properties will always take precedence
    // over spread ones, regardless of order.
    let spread_house = House {
        color: String::from("white"),
        bathrooms: 3,
        ..built_house
    };

    println!(
        "I built this house with a spread operator. It is a {} {}bed/{}bath worth ${:.2}.",
        spread_house.color,
        spread_house.bedrooms,
        spread_house.bathrooms,
        spread_house.price_in_cents / 100
    );

    // This is invalid b/c the spread operator MOVES in the String value of color.
    // If we only spread in stack values, we would not have an issue because
    // stack values implement COPY, not MOVE.
    // let spread_house = House {
    //     bedrooms: 10,
    //     ..built_house
    // };
    // println!("{}", built_house.color);

    // Tuple structs are also a thing. Easy way to buncle related data.
    let location = Coordinates(5, -32);
    println!(
        "You can also use tuple structs to bundle data: X:{} Y:{}",
        location.0, location.1
    );

    // This appears to be explained more later but this is a unit struct
    // Essentially an empty object with a type.
    let subject = AlwaysEqual;
}

// unti struct
struct AlwaysEqual;

// tuple struct
struct Coordinates(i32, i32);

// standard struct
// you can also define properties as references, which would mean ownership of the
// property is somewhere else, but you need to define a lifetime. Lifetimes are explained later.
struct House {
    color: String,
    bedrooms: u32,
    bathrooms: u32,
    price_in_cents: u64,
}

fn build_house(color: String, bedrooms: u32, bathrooms: u32) -> House {
    let price = bedrooms * 30_000 + bathrooms * 15_000;
    // we can assign param names and values at the same time fi they are equal
    House {
        color,
        bedrooms,
        bathrooms,
        price_in_cents: (price * 100) as u64,
    }
}
