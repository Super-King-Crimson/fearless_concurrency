use crate::{thread, mpsc, Arc};

pub fn explain() {
    println!("Send indicates that ownership can be safely transferred between threads");

    //Almost all Rust types are send, with a few exceptions like:
        //Rc, which may miscount if it receives two clones or drops at once
            //this could lead to an accidental double free or memory leak
        //instead use Arc, which does implement Send

    //Luckily, trying to use an non-Send trait in a thread is banned by the compiler
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        //tx.send(Rc::new(10));
        tx.send(Arc::new(10)).unwrap();
    }).join().unwrap();

    println!("{}", rx.recv().unwrap());
}