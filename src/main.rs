use reqwest;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    hammer(0);
}

fn hammer(num: i32) {
    let counter = Arc::new(Mutex::new(num));
    let mut handles = vec![];

    for _ in 0..rayon::current_num_threads() {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            reqwest::get("https://api.poempath.com/randomPoem")
                .unwrap()
                .text()
                .unwrap();

            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Requests Sent: {}", *counter.lock().unwrap());

    hammer(*counter.lock().unwrap());
}
