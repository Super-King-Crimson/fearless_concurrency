mod mutex;
mod atomics;
mod mutex_vs_refcell;

pub fn explain() {
    println!("Message passing isn't the only way to handle concurrency");
    //Remember 'share memory by communicating'? Another way to handle concurrency is
        //to let multiple threads access the same shared data

    println!("But what would the issue with shared memory be?");
    //Channels are similar to single ownership: only one thread will own a piece of data at a time
    //But if memory is shared, there are multiple owners to data, so we need smart pointers

    //Let's talk about mutex for shared memory
    mutex::explain();
    atomics::explain();
    mutex_vs_refcell::explain();
}