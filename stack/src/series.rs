
pub fn fibonacci(nvalue : u32) -> u32 {
    println!("value :{}", nvalue);
    if nvalue <= 1 {
        return nvalue
    }    
    return fibonacci(nvalue -1) + fibonacci(nvalue -2 )
}