fn main(){
    let greeting=String::from("Hello World");
    println!("{}",greeting);

    let char1:Option<char>=greeting.chars().nth(0);

    match char1 {
        Some(c)=>println!("First character is {}",c),
        None=>println!("String is empty"),
    }
}