use crate::{Arc, Mutex, thread, Duration};
use std::time::SystemTime;

pub fn explain() {
    println!("Look back at atomics.rs for a second");

    //here are a couple things to note:
    //Note our Mutex<Arc<T>> structure to allow for mutability between threads
        //That's a lot like Rc<RefCell<T>>
    //So what other similarities to these two things have?
    exhibit_a();
    exhibit_b();
}

fn exhibit_a() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);

    //Notice how we get a mutable reference from an immutable borrow
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
        
    println!("{num}");

    //That means that Arc<Mutex<T>> also gives us interior mutability
    //Unfortunately, that also means you don't get Rust's cool compile time borrow checking
}

#[allow(unused)]
fn exhibit_b() {
    //Remember how Rc's can cause reference cycles?
    //Mutex's can cause deadlocks, when an operation needs to lock two resources
        // and two threads have each acquired one lock, causing them to wait for each other forever

    //Here's an example:

    println!("\nWELCOME TO THE BIT FLIPPING CHARACTER EATING EXTRAVAGANZA");

    //Let's make two Arc<Mutex<_>> types, one containing a string, the other a bool
    let give_me_chars = Arc::new(Mutex::new(String::new()));
    let toggle_me = Arc::new(Mutex::new(false));

    //We're going to make two threads here so lets make some clones
    let char_clone = Arc::clone(&give_me_chars);
    let char_clone2 = Arc::clone(&give_me_chars);

    //This will allow our threads to share some data
    let toggle_clone = Arc::clone(&toggle_me);
    let toggle_clone2 = Arc::clone(&toggle_me);

    //This thread will lock and start working with char Mutex, and when it's done,
    //it should UNLOCK it's mutex and wait for the bool Mutex to unlock so it can work on that
    let char_thread = thread::spawn(move || {
        println!("Doing calculations to push some strs");
        thread::sleep(Duration::from_millis(500));
        
        let mut adder = char_clone.lock().unwrap();

        adder.push('か');
        thread::sleep(Duration::from_millis(800));
        adder.push('B');
        thread::sleep(Duration::from_millis(300));
        adder.push('&');

        loop {
            println!("Waiting 500 ms for flipper lock to end");
            thread::sleep(Duration::from_millis(500));

            match toggle_clone2.try_lock() {
                Ok(mut val) => {
                    //flip the bool if the current number of milliseconds since epoch is an even number
                    //if the current time is before the epoch then we have bigger problems don't flip
                    if let Ok(secs) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|dur| dur.as_millis()) {
                        if secs % 2 == 0 {
                            *val = !*val;
                        }
                    }
                }

                Err(_) => (),
            }
        }
    });

    //This thread locks and does work with the bool Mutex, and when it's done,
    //it should UNLOCK it's mutex and wait for the char Mutex to unlock so it can work on that
    let toggle_thread = thread::spawn(move || {
        println!("Doing very important bit-flipping calculations");

        let mut toggler = toggle_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(1));

        for _ in 0..5 {
            *toggler = !*toggler;
            thread::sleep(Duration::from_millis(50));
        }

        loop {
            println!("Waiting 500 ms for char lock to end");
            thread::sleep(Duration::from_millis(500));

            match char_clone2.try_lock() {
                Ok(mut str) => {
                    println!("I'm breaking the rules, i'm putting on a &str!");
                    str.push_str("ITHINKIMDOINGIT");

                    str.push_str(" phew did it thank god that was hard");
                    
                    //Hey, I'm a hint! You might want to drop a break here somewhere.
                    //You also might want to 'drop' something else...
                }

                Err(_) => (),
            }
        }
    });

    //If either of these threads fail to unlock their locks when they're done with them,
        //DEADLOCK: program will NEVER finish
    
    //Alternatively: if one thread forgets to insert a break statement in their loop 
        //that checks if the other Mutex has unlocked yet
    //That's not a deadlock, but it will cause the code to never finish
        //Make Sure To Break Your Loops! 
            //(wait i could have just done while let bruhhhhhhhhhhh)

    //let the threads run to (in)completion
    char_thread.join().unwrap();
    toggle_thread.join().unwrap();

    //And print our results
    println!("toggler: {} | char eater: {}", toggle_me.lock().unwrap(), give_me_chars.lock().unwrap());
}