use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let first_part = false;
    if first_part {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();

        let v = vec![1, 2, 3];

        let another_handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        another_handle.join().unwrap();
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
