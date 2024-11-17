use rand::Rng;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let threads = 16;
    let attempts = Arc::new(Mutex::new(0));
    let password = "%!11".to_string();
    let password_len = password.len();
    let mut symbols = Vec::new();

    for i in 0..128 {
        let value = char::from_u32(i);
        match value {
            Some(symbol) => symbols.push(symbol),
            _other => {}
        }
    }

    let now = Instant::now();

    let mut handles = vec![];
    for _ in 0..threads {
        let symbols = symbols.clone();
        let password = password.clone();
        let attempts = Arc::clone(&attempts); // Clone Arc for thread-safe access

        let handle = thread::spawn(move || {
            function(&symbols, &password, password_len, attempts);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{0} == {0}", password);

    let elapsed = now.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;
    let attempts = attempts.lock().unwrap();
    println!("Elapsed: {}m {}s; Attempts: {}", minutes, seconds, *attempts);
}

fn function(symbols: &[char], p: &String, p_len: usize, a: Arc<Mutex<i32>>) {
    loop {
        let mut attempt_list = Vec::new();
        for _ in 0..(p_len as i32) {
            attempt_list.push(symbols[rand::thread_rng().gen_range(0..symbols.len())].to_string());
        }

        let attempt = attempt_list.join("");

        {
            let mut a = a.lock().unwrap();
            *a += 1;
        }

        if attempt == *p {
            break
        }
    }
}