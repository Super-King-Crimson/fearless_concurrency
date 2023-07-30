use crate::{Duration, thread};

pub fn explain() {
    println!("Oh my GOD it's task.spawn");

    //Use thread::spawn with a closure with the code you want to run in the thread
    let thread = //TODO uncomment this line to contine
    thread::spawn(|| {
        for i in 1..=10 {
            println!("This is number {i} speaking from spawned thread come in");
            if i == 6 {
                println!("...hello?");
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=5 {
        thread::sleep(Duration::from_millis(50));
        println!("Hello number from thread this is {i} from main thread");
        thread::sleep(Duration::from_millis(50));
    }

    //Even running this a couple times you'll see how inconsistent threads are
    //also the spawned thread immediately died the second the main thread ended
    //run this a couple times then uncomment these two lines and 
    //TODO Uncomment these
    thread.join().unwrap();
    elaborate();
}

#[allow(unused)]
fn elaborate() {
    println!("\n\n");
    println!("We probably do want that spawned thread to finish though so...");

    //note the type JoinHandle that returns from thread::spawn
    let thread = thread::spawn(|| {
        for i in 0..10 {
            println!("{i}");
            thread::sleep(Duration::from_millis(40));
        }

        println!("ok im done sorry for blocking you");
    });

    for i in 0..10 {
        println!("Hey this is {i} from main thread, ya done yet spawned thread?");
        thread::sleep(Duration::from_millis(10));
    }

    println!("ok you're taking too long i'll just wait");

    //by calling this the main thread will block the current thread until the thread in this handle finishes
    thread.join().unwrap();

    //see this doesn't print until the loop is done and "sorry for blocking you" is printed
    println!("ok cool");
}
