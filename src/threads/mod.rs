pub mod creation;
pub mod move_closures;

pub fn explain() {
    println!("Threads run code simultaneously");

    //In most operating systems, an executed program's code is run in a process,
    //and the operating system will manage multiple processes at once

    //The feature that runs these processes are called threads
        //A web server may have multiple threads so it can respond to multiple request simultaneously

    println!("Splitting computation in a program into threads isn't all sunshine and rainbows");
    
    //Threads add complexity: now there's no guarantee which thread will compute first
    //This can cause:
        //Race conditions: threads access data in an inconsistent order
        //Deadlocks: two threads are waiting for each other to finish, and therefore will never continue
        //Bugs that only happen in specific scenarios (like in a specific race condition)
    //No matter how fearless Rust's concurrency is,
    //multi-threaded programming takes careful thought and a different structure than single-threaded

    //Rust programming uses a 1:1 model: so one Rust thread = one operating system thread
    creation::explain();
    move_closures::explain();
}