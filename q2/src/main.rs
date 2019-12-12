fn main() {
    fn do_thrice<F:FnMut()>(mut func: F)
    {
        func();
        func();
    }
    let mut num: usize = 1;
    {
        let add_three_to_num = || num += 3;
        do_thrice(add_three_to_num);
    }
    println!("{}",num);
    }
    
    
    
    
    
    