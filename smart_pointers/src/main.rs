use crate::LList::{List, Nil};
use std::{fmt::Display, ops::Deref, rc::Rc};

fn main() {
    let my_box = Box::new(100);
    println!("myBox = {}", my_box);

    let list_example = List(1, Rc::new(List(2, Rc::new(List(3, Rc::new(Nil))))));
    println!("Linked List = {:?}", list_example);

    let a = 5;
    let b = &a;

    println!("a = {}", a); // value of a is 5
    println!("*b = {}", *b); // value of the pointer that points to a is 5

    let x = 10;
    let y = Box::new(x);

    println!("x = {}", x); // value of x is 10
    println!("*y = {}", y); // value of the Box that contains a copy of x is 10

    let foo = 25;
    let bar = MyCustomSmartPointer::new(foo);

    println!("foo = {}", foo);
    // this works for our custom type b/c we implemented the Deref trait
    // under the hood, it does this: *(bar.deref())
    println!("*bar = {}", *bar);

    // because our customer smart pointer implements a generic type and the Deref trait,
    // the compiler can infer derefs across types... in this case String and str. String derefs
    // into &str and the compiler knows that, so this is valid. Withoug deref coercion, it would
    // look like this: hello(&(*s)[..]); GROSS
    let s = MyCustomSmartPointer::new(String::from("World!"));
    hello(&s);

    // we can drop memory manually if needed
    let z = MyCustomSmartPointer::new(true);
    drop(z);
    println!("d is dropped");

    // Rc is like Box, but it only drops when all of its references are gone. It allows
    // sharing of ownership. The are always immutable!
    let list_a = Rc::new(List(5, Rc::new(List(10, Rc::new(Nil)))));
    let list_b = List(3, Rc::clone(&list_a));
    let list_c = List(4, Rc::clone(&list_a));
    println!("Linked List A = {:?}", list_a);
    println!("Linked List B = {:?}", list_b);
    println!("Linked List C = {:?}", list_c);

    let list_x = Rc::new(List(5, Rc::new(List(10, Rc::new(Nil)))));
    println!("Rc count after creating x = {}", Rc::strong_count(&list_x));
    let list_y = List(3, Rc::clone(&list_x));
    println!("Rc count after creating y = {}", Rc::strong_count(&list_x));
    {
        let list_z = List(4, Rc::clone(&list_x));
        println!("Rc count after creating z = {}", Rc::strong_count(&list_x));
    }
    println!(
        "Rc count after z goes out of scope = {}",
        Rc::strong_count(&list_x)
    );
}

fn hello(s: &str) {
    println!("Hello, {}", s);
}

// cons is a lip concept used to represent linkedlists, we use it here to demostrate
// dynamically sized types and how Box solves them
#[derive(Debug)]
enum LList {
    List(i32, Rc<LList>),
    Nil,
}

struct MyCustomSmartPointer<T: Display>(T);

impl<T: Display> MyCustomSmartPointer<T> {
    fn new(x: T) -> MyCustomSmartPointer<T> {
        MyCustomSmartPointer(x)
    }
}

// DerefMut also exists for dereferences mutable values
impl<T: Display> Deref for MyCustomSmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyCustomSmartPointer<T> {
    // this will be called everytime a MyCustomSmartPointer goes out of scope
    fn drop(&mut self) {
        println!("Dropping MyCustomSmartPointer: {}", &self.0);
    }
}
