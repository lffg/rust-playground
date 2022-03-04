use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

const STARTING_POINT: u64 = 15;

#[derive(Debug)]
enum Command {
    Incr,
    CollatzAsEven,
    CollatzAsOdd,
    Fetch(Sender<u64>),
}

fn main() {
    let (b_cmd_sender, a_recv) = channel();
    let c_cmd_sender = b_cmd_sender.clone();

    let a = spawn(move || {
        let mut i = STARTING_POINT;

        while let Ok(command) = a_recv.recv() {
            println!("[a] Received {command:?}:");
            println!("[a]     Before: `{i}`");
            use Command::*;
            match command {
                Incr => {
                    i += 1;
                }
                CollatzAsEven => {
                    i /= 2;
                }
                CollatzAsOdd => {
                    i = i * 3 + 1;
                }
                Fetch(sender) => {
                    sender.send(i).expect("Couldn't fetch: receiver hung up.");
                }
            }
            println!("[a]     After: `{i}`")
        }
        println!("[a] Done.");
    });

    let b = spawn(move || {
        let (tx, rx) = channel();

        println!("[b] Sending fetch command...");
        b_cmd_sender.send(Command::Fetch(tx)).unwrap();
        let fetched_i = rx.recv().expect("Couldn't fetch: sender hung up.");
        println!("[b] Received {fetched_i}.");

        sleep_ms("b", 2000);

        let collatz_cmd = if fetched_i % 2 == 0 {
            Command::CollatzAsEven
        } else {
            Command::CollatzAsOdd
        };
        println!("[b] With i as {fetched_i}, decided to run collatz with {collatz_cmd:?}.");
        b_cmd_sender.send(collatz_cmd).unwrap();
        println!("[b] Done.");
    });

    let c = spawn(move || {
        sleep_ms("c", 1000);

        println!("[c] Will send incr command...");
        c_cmd_sender.send(Command::Incr).unwrap();

        println!("[c] Done.");
    });

    a.join().unwrap();
    b.join().unwrap();
    c.join().unwrap();
}

fn sleep_ms(thread_id: &'static str, ms: u64) {
    use std::{thread::sleep, time::Duration};

    println!("[{thread_id}] Will sleep ({ms} ms)...");
    sleep(Duration::from_millis(ms));
    println!("[{thread_id}] Woke up.");
}
