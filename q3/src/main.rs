use std::thread;
use std::time::Duration;

fn main () {
    let handle = thread::spawn (|| {
        for a in 1..8 {
            println!("hello {} from spawned thread!",a);
            thread::sleep(Duration::from_millis(2));
        }

    });
    
    for b in 1..6 {
           println!("hello {} from main thread!",b);
            thread::sleep(Duration::from_millis(2));
    }
    
    
    handle.join().unwrap();
    
    
}