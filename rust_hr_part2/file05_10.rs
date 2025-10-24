use std::collections::HashMap;



fn main()
{
    let mut scores=HashMap::new();

    scores.insert("Alice",50);
    scores.insert("Bob",40);
    scores.insert("Charlie",30);

    println!("Iterating over key-value pairs : ");

    for (key,value) in scores.iter() {
        println!("{}: {}",key,value);
    }

    //Ex2: Iterating over mutable variables
    println!("Iterating over mutable key-value pairs: ");
    for (key,value) in scores.iter_mut() {
        *value+=10;
        println!("{}: {}",key,value);
    }
}