use crate::{Duration, mpsc, thread};

pub fn explain() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vec = vec![1, 2, 3, 4, 5];
        
        for v in vec {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    //notice how we don't call rx.recv or rx.try_recv
    //We can use rx as a iterator:
        //when a value is received, it's printed
        //when the transmitting channel closes, the iterator ends
    //Since this waits without a thread::sleep,
        //we know that this iterator yields to receive values
    for received in rx {
        println!("Got: {}", received);
    }
}