// use std::sync::{Mutex, Arc};
use d_i::{service};
// use thread_sync::{test_di, get_instance};

pub mod thread_sync;
pub mod d_i;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("{}", thread::available_parallelism()?);
//     let spawned = thread::spawn(|| start_count("thread"));
//     start_count("main");
//     spawned.join().unwrap();
//     Ok(())
// }

fn main() {
    // let container = Arc::new(Mutex::new(test_di().unwrap()));
    // get_instance(&container).unwrap();
    service(); 
}
