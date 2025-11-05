use std::{sync::mpsc::channel, thread};

fn main() {
    let (sender, receiver) = channel::<String>();

    // here I moved the sender to the thread
    thread::spawn(move || {
        sender.send(String::from("yoo")).unwrap();
    });

    let value = receiver.recv().unwrap();

    // unwrap means if something returing Result enum and u dont care about err or use pattern matching so that to do something else when err occurs

    println!("{}", value);

    //* Use Mutiple threads to do some opertation and send data to the single sender */
    let (tx, rx) = channel();

    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..100 {
                ans = ans + (i * 100 + j);
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx);

    let mut ans: u64 = 0;

    for val in rx {
        ans = ans + val;
        println!("value received {}", val);
    }

    println!("Ans is {}", ans);
}
