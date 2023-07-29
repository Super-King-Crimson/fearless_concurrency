use crate::thread;

pub fn explain() {
    println!("Usually the move keyword is passed to closures in thread::spawn");
    
    //This transfers ownership from one thread to another
    let v = vec![1, 2, 3];
    
    //yeah you have to move the vec to print it, we can't even take a reference
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    }).join().unwrap();

    //this doesn't work originally because 
    //the thread tries to borrow &v, but if the main thread drops v while/before the thread prints it,
    //that's undefined behavior

    //we can't even intentionally screw ourselves over. Darn it Rust!
    // drop(v);
}