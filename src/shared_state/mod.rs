pub fn explain() {
    println!("Message passing isn't the only way to handle concurrency");
    //Remember 'share memory by communicating'? Another way to handle concurrency is
        //to let multiple threads access the same shared data

    println!("But what would the issue with shared memory be?");
    //Channels are similar to single ownership: only one thread will own a piece of data at a time
}