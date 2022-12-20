use std::fmt::Display;

// traits are similar to interfaces in other languages. They are a set of behavior that a type
// must implement.
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("I am out of ideas for horse books. We have to shut down."),
        reply: false,
        retweet: false,
    };
    let article = Article {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Hugh Mungus"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("SUMMARIZE");
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());

    println!("NOTIFY");
    notify(&tweet);
    notify(&article);
}

// This trait requires anything that implements it to have a method named summarize that matches
// the signature.
trait Summary {
    // by not providing a default implementation here, we are requiring that any type that
    // implements this trait to provide its own implementation of summarize_author.
    fn summarize_author(&self) -> String;

    // this can also implement default behavior for a trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// You can also use traits as parameters to functions. The parameter must be a reference to
// any implementation of the trait.
// You can also imdicate multiple traits with a + sign and where.
fn notify<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summarize());
}

// functions that pass in traits as parameters are actual syntax sugar for something
// call a trait bound. Shown in function below:
// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// You can also use traits as return types. However, you cannot return different types.
// The return type must be singularly concrete, otherwise It will not compile.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // this would not compile
    // why this is and how to fix it is covered in ch17
    // if switch {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from(
    //             "of course, as you probably already know, people",
    //         ),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}

struct Article {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// this will use the default implementation of summarize
impl Summary for Article {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// This is how you implement a trait for a type. The type must match the trait signature.
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// this will only work if T implements the PartialOrd trait. It conditionally
// implements the cmp_display method when that condition is met.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// you can also add an impl for any implementation of a trait. This is called a blanket implementation.
// Below is an example of how the standard libary does a blanket implementation for toString
// on anything that implements the Display trait.
// impl<T: Display> ToString for T {
//     // --snip--
// }
