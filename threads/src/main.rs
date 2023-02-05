use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn create_process(name: &str, num: u64) -> impl Fn() -> () {
    let s = String::from(name);
    return move || {
        println!("{s}: Starting New Process...");
        for i in 0..num {
            thread::sleep(Duration::from_secs(1));
            println!("{s}: {} seconds have passed...", i + 1);
        }
        println!("{s}: Stopping Process...");
    };
}

fn main() {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let processes = vec![
        create_process("Process 1", 2),
        create_process("Process 2", 4),
        create_process("Process 3", 6),
        create_process("Process 4", 8),
        create_process("Process 5", 10),
    ];

    for p in processes {
        let t = thread::spawn(p);
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}
