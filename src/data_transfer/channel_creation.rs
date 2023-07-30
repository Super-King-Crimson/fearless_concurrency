use crate::thread;
use crate::mpsc;
use crate::Duration;

pub fn explain() {
    //Quick note: a channel is considered 'dead' if the receiver or transmitter threads finish executing
    println!("We have to make a program that has a transmitter thread and a receiver thread");

    //We do this by using the mpsc::channel fn
    //this also needs type params, rust can't just infer off the bat
    let (tx, rx) = mpsc::channel();
    //mpsc stands for 'multiple producer, single receiver:' 
        //rust channels can have a lot of transmitters but only one receiver to consume values
    //returns a tuple: first element is transmitter (tx), second element is receiver (rx)

    //Note the move closure: we have to move tx into the thread to use it
    thread::spawn(move || {
        let val = String::from("hi");
        //this kinda looks like thread.join().unwrap(), but we have a parameter: val, the value we send
        tx.send(val).unwrap();
        //It even returns a Result<T, E>
            //this returns Err if our reciever thread doesn't exist or smthn else went wrong
        
        thread::sleep(Duration::from_secs(2));
        tx.send(String::from("here's some more data")).unwrap();
        //and you send can multiple pieces of data per thread

        //Also note that a channel can only send one type, so this is an error
        // tx.send("hi").unwrap();
    });

    //speaking of join, the method that blocks execution until a thread is done running
    //recv blocks execution until a channel sends a message down the channel
    //This will return Err<E> when all transmitters finish execution (ie when all transmitters close)
        // If there are no more places to send values from, then no more values will be sent (obviously!)
    let received = rx.recv().unwrap();
    println!("value received: {received}");

    //There's also a try_recv method which will try to get a value from a channel immediately (non-blocking)
    
    //That means we can do something like:
    loop {
        //see if some value is available yet
        match rx.try_recv() {
            //if it is, go use it
            Ok(str) => {
                println!("Value received: {str}");
                break;
            }

            //if it isn't, work on something else for a bit while waiting for the value
            Err(_) => {
                println!("Value not received yet, working on other stuff for a bit");
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}