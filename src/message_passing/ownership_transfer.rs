use crate::{thread, mpsc};

pub fn explain() {
    println!("Ownership rules in threads are similar to general memory rules");
    
    //Here let's try some stuff
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        //Nope nope nope that's banned
        // println!("{val}");
        //We no longer have ownership of val, it was sent
        //What would happen if the thread we sent this value dropped this, 
            //what would this print?
        //and that's a COMPILE TIME error. ain't that awesome?
    });

    let data = rx.recv().unwrap();
    println!("{data}");
}