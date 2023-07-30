pub mod channel_creation;
pub mod ownership_transfer;
pub mod iterative_sends;
pub mod multiple_producers;

pub fn explain() {
    println!("One popular approach to ensure safe concurrency is message passing");
    //This is when threads or actors communicate by sending each other messages containing data

    println!("Do not communicate by sharing memory, share memory by communicating");
    //If you have one program that runs independently, it needs no special communication threads
    //If you have another program that runs independently, it also needs no special communication threads
    //If you link them, and that link includes communication, and that communication includes code to share data, 
        //you STILL need no special communication threads (i think???)

    //Rust's std has an implementation of channels: a way to send data from one thread to another
    //Channels are like river: you put something in, and it goes somewhere else for someone else to use

    //A channel has a transmitter and a receiver
    //Let's channel some receivers
    channel_creation::explain();
    ownership_transfer::explain();
    iterative_sends::explain();
    multiple_producers::explain();
}