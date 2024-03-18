use crate::series::fibonacci;

mod collection;
mod series;
use std::{io::{self, Read}, result};

fn main() {

    /* 
    
       */
      callfibonacci()
}

fn callfibonacci(){
    let mut inp = String::new();
    println!("Reading std io");
    let _read_to_string = io::stdin().read_to_string(&mut inp);
    println!( "Entered value {}",inp);
    let val = inp.parse::<u32>().unwrap();
    let result = fibonacci(val);

    println!("Fibonacci value of {} is {}",val,result)
}

fn store() {
    let mut store: collection::Store<i32> = collection::Store::new();
    for std::ops::Range { start, end } in [1..10]  {
          store.insert(todo!());        
    }

    if let Some(item) = store.get() {
        
    }
    
}
