use core::num;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};
pub fn take_turns() {
    thread::spawn(|| {
        for i in 1..100 {
            println!("Hello from child: {i}");

            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..15 {
        println!("Hello from momma: {i}");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn join() {
    let handle = thread::spawn(|| {
        for i in 1..3 {
            println!("Hello from child: {i}");

            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..15 {
        println!("Hello from momma: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap()
}

pub fn nest() {
    let mut n = 1;
    println!("1:{n}");
    let t = thread::spawn(move || {
        n = n + 1;
        println!("2:{n}");
        thread::spawn(move || {
            n = n + 1;
            println!("3:{n}");
        })
    });
    n = n + 1;
    println!("5:{n}");
    t.join().unwrap().join().unwrap();
    println!("6:{n}");
}

pub fn channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    for msg in &rx {
        println!("Hello from rx: {msg}");
    }
}

pub fn channel_wait() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in &rx {
        println!("Hello from rx: {msg}");
    }
}

pub fn channel_multi() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("u"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for msg in &rx {
        println!("Hello from rx: {msg}");
    }
}

pub fn single_thread_mutex() {
    let m = Mutex::new(6);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

pub fn multi_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
