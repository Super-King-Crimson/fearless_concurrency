use crate::{mpsc, Duration, thread};

pub fn explain() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("this"),
            String::from("is"),
            String::from("from"),
            String::from("a"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let t_f = [true, false, false, false];

        for (i, bool) in t_f.iter().enumerate() {
            tx1.send(format!("condition {i} is {bool}")).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    //The order is inconsistent, it depends on the operating system running
    for received in rx {
        println!("Received: {received}");
    }
}