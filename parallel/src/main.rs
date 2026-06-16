use std::thread;
use std::time::Duration;

fn main() {
    let name = String::from("Nabil");

    let handle: thread::JoinHandle<u32> = thread::spawn(move || {
        println!("Worker Thread: Starting....");
        println!("Name {name}");
        thread::sleep(Duration::from_secs(1));
        let result = (1..=10).sum::<u32>();
        println!("Worker Thread: Done Result....{result}");
        result
    });

    println!("🏠 Main thread: doing other stuff while worker runs...");
    thread::sleep(Duration::from_millis(500));
    println!("🏠 Main thread: now waiting for the worker...");

    let result = handle.join().expect("Thread panicked");
    println!("Main Thread Got the result {result}");
}
