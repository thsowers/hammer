use reqwest;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    hammer(0);
}

fn hammer(num: usize) {
    let mut handles = vec![];

    for _ in 0..rayon::current_num_threads() {
        let handle = thread::spawn(move || {
            reqwest::get("https://someones.url.net")
                .unwrap()
                .text()
                .unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Requests Sent: {}", num);
    hammer(rayon::current_num_threads() + num);
}
