#![allow(unused)]
use crate::{Rc, Mutex, thread, Arc};

pub fn explain() {
    //let counter = Mutex::new(0);
    //let counter = Rc::new(Mutex::new(0));

    //Ok NOW we have the right types
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    //We make 10 threads, have them all increment the lock by one, and push their handlers onto a vec
    for _ in 0..10 {
        //Now, instead we make a different owned Rc for mutex, and have the closure own that
        // let counter_clone = Arc::clone(&counter);
        //Ok but let's have it of an actual thread safe type
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            //ignore this look at what was there before
            //This complains 'moved value'
            // let mut num =  counter.lock().unwrap();
            //That's because this gets called multiple times in the loop,
                //meaning the mutex is moved into and owned by multiple closures (you see the issue?)
            //That means we need to somehow own this Mutex multiple times, so let's use Rc
    
            //Oh shoot this doesn't work either, apparently Rc's don't work between threads
            //(apparently it doesn't implement some Send trait?)
            //let mut num = counter_clone.lock().unwrap();
            //Rc counts references, but it doesn't check concurrency 
                //to make sure its counts can't be overrided or interrupted by another thread
            //Instead, we use Arc<T>, which is just Rc<T> but thread safe (the A stands for atomic), so
                //AtomicallyReferenceCounted(Pointer)<T> (check the sdocs at std::sync::atomic)

            //While I pub use Arc into my this crate, 
                //let's wonder why there are thread safe and thread unsafe versions of types at all?
                //Why not just have std type that works with threads?
            //Thread safety comes with a performance penalty 
                //that you only want to pay when you really need to.
            //Basically only write concurrent Rust if you REALLY need the performance boost,
                //as if you're performing operations within a single thread.
                //your code may run faster if you don't have to abide by the atomics rules

            //Ok That Took Forever Here We Go!!!
            let mut num = counter_clone.lock().unwrap();

            *num += 1;

            //Nailed It.
        });

        handles.push(handle)
    }

    //Wait for all handles to finish
    for handle in handles {
        handle.join().unwrap();
    }

    //And print our result
    println!("Result: {}", *counter.lock().unwrap());
    //Now we can count to 10 at the speed of Light
    
    println!("Obviously you can do WAYYY more complex things with Mutex");
    //We can now divide a calculation into multiple parts, split that across threads
        //and use a Mutex<T> to combine all of those results together
    
    println!("Actually, if you're working with simple numbers, you can use a simpler type");
    println!("Check out std::sync::atomic!");
}