pub use std::thread;
pub use std::time::Duration; 
pub use std::sync::mpsc;

pub mod topic {
    pub fn introduce() {
        println!("\
            ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~Chapter 16: FEARLESS CONCURRENCY~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ \n\
            - Concurrent programming and parallel programming are becoming \
            more and more important as computers use their multiple processors \n\
            \t- Concurrent programming = different parts of a program execute independently \n\
            \t- Parallel programming = different parts of a program execute at the same time \n\
            - Rust's type and ownership systems can also solve concurrency problems \n\
            \t- i.e. concurrency errors are caught at ompile-time, so Rust won't compile them \n\
            - We will be learning: \n\
            \t- Thread creation to run multiple blocks of code parallel \n\
            \t- Message-passing concurrency to send messsages between threads \n\
            \t- Shared-state concurrency to allow multiple threads to access some piece of data \n\
            \t- Sync and Send, traits to extend Rust's concurrency to user-defined types \n\
            - Note: 'concurrent' = 'concurrent and parallel'
        ")
    }
}

pub mod threads;

pub mod message_passing;

pub mod shared_state; 