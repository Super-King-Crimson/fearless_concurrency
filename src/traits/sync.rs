pub fn explain() {
    println!("Sync allows referencing from multiple threads");

    //A type T is Sync if &T is Send, 
        //i.e. if a reference to it can be safely sent to another thread

    //All primitives are Sync, all types (enums and structs) made of Sync types
        //are also Sync (note that this rule also applies with Send) dk if i mentioned that
    
    println!("Sync == thread-safe");
    
    //Why is Send and Sync two different traits though?
        //Some values are one, both, or neither. Let's go through some now:
    
    //Rc:           neither, for reasons previously described.
    //RefCell:      Send, but not Sync. The runtime borrow checker for it is not thread-safe
    //Mutex:        Send and Sync, has atomic borrow checking detection
    //MutexGuard:   Sync, but not Send, as some platforms require a Mutex be unlocked by the thread that locked it 
}