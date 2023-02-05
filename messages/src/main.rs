use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

fn create_transmitter(tx: &Sender<String>, msg: &str) -> impl Fn() -> () {
    let tx_clone = tx.clone();
    let msg_clone = String::from(msg);
    move || {
        let words = msg_clone.split_whitespace();
        for word in words {
            thread::sleep(Duration::from_secs(1));
            tx_clone.send(String::from(word)).unwrap();
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let processes = vec![
        create_transmitter(&tx, "Hello, World!"),
        create_transmitter(&tx, "Goodbye, World!"),
        create_transmitter(&tx, "Yo Quiero Taco Bell!"),
    ];

    for p in processes {
        thread::spawn(p);
    }

    for received in rx {
        println!("Got: {}", received);
    }
}
