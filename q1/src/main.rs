fn main() {
    fn consume_with_relish<F:FnOnce()->String>(func: F)
         where F: FnOnce() -> String
    {
      
       
    
        println!("Consumed: {}", func());
    
        println!("Super Happy I did it!");
    }
    // fn once function always consume values whether it is in println form or in its variable. it capture values from environment
    // thats the reason i wrote print line after scope...
    println!("Super Happy I did it!");
    }