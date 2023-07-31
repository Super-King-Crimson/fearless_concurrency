use crate::{Mutex, MutexGuard};

pub fn explain() {
    println!("Hello mutexes are pretty great and thread-safe");
    //Mutex stands for mutual exclusion
    //To access data in a mutex, thread must signal that it wants data and get the mutex's lock
    //A mutex's lock is a data structure that keeps track of the items currently accessing the data

    //To use a Mutex, remember these rules:
        //Acquire lock before using data
        //When done with data, return the lock so other threads can access the data
    
    //Think of it like a conference where there's only one microphone
        //For other speakers to talk, they have to signal that they want the microphone,
        //then they can talk as long as they want
    //Once they're done talking, they have to hand it off to other speakers that want to speal
        //If a speaker forgets to hand off the microphone, the panel will suck
    
    //Mutexes are pretty hard, but come on this is Rust you can handle hard stuff
    let m = Mutex::new(5);

    {
        //Can be used as a normal pointer once locked, this will panic if the mutex is already locked
        //MutexGuard is a smart pointer
        let mut num: MutexGuard<'_, i32> = m.lock().unwrap(); 
        *num = 6;
    } //automatically unlocked (dropped) when the lock goes out of scope

    //WTH, POISONED???
    //Since this has been unlocked we can now access the data in the Mutex
    println!("m = {m:?}");
}