use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn build_counter(m: &Arc<Mutex<i32>>) -> impl Fn() -> () {
    let m_clone = Arc::clone(m);
    move || {
        let mut val = m_clone.lock().unwrap();
        println!("Adding 1 to {val}");
        *val += 1;
    }
}

fn main() {
    // Rc is not thread safe, and the compile will not allow it to be used to share
    // data. Use the Arc structure instead when you need to share state across threads.
    // Arc is less performant than Rc, so you should use Rc whenever possible.
    let number = Arc::new(Mutex::new(0));

    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let processes = vec![
        build_counter(&number),
        build_counter(&number),
        build_counter(&number),
        build_counter(&number),
        build_counter(&number),
    ];

    for p in processes {
        let t = thread::spawn(p);
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Value is {}", number.lock().unwrap());
}
