use fearless_concurrency::*;

fn main() {
    println!("Moved back to mod.rs syntax for modules, hope it's worth it");

    topic::introduce(); 
    // threads::explain();
    // message_passing::explain();

    //Note: There's some buggy code in src/shared_state/mutex_vs_refcell, in the exhibit_b fn.
    // shared_state::explain();
    //If you want to try to bugfix it, go ahead!
    //If you don't, just comment out the code that calls it in src/shared_state/mutex_vs_refcell, in the explain fn.
    //Keep getting your Rust up!

    traits::explain();
}