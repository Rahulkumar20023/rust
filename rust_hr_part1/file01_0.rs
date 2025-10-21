fn main(){
    let greeting=String::from("Hello World");
    println!("{}",greeting);

    let greeting2:&str="Hello World 2";
    println!("{}",greeting2);
}