pub mod send;
pub mod sync;

pub fn explain() {
    println!("Rust has VERY little built-in concurrency");

    //mpsc and all that junk is a part of the standard library (std), not the language itself
    //However, Rust has TWO features embedded into the language: Sync, and Send (std::marker)
        // These are traits that allow you to make custom concurrency
    send::explain();
    sync::explain();

    //Oh one very important thing: DO NOT MANUALLY IMPLEMENT SEND AND SYNC
        //Send and Sync are just marker traits: they don't even have any methods
    
    //To manually implement these traits, you need unsafe Rust 
        //Check out the Rustonomicon to learn how to uphold the safety guarantees of Sync and Send
            //or don't if you want to continue touching grass
}