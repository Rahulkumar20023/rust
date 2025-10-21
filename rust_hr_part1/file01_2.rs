fn main(){
    

    let greeting2:&str="Hello World 2";
    println!("{}",greeting2);

    let ch:Option<char>=greeting2.chars().nth(0);

    match ch {
        Some(c)=>println!("First character is {}",c),
        None=>println!("String is empty"),
    }
}