mod collection;

fn main() {
    let mut store: collection::Store<i32> = collection::Store::new();
    for count in [1..10]  {
          store.insert(count);        
    }

    while !store.is_empty() {
        let item: = store.get();
        println!("{}",item.unwrap())
    }
       
}
