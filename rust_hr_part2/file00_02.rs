fn get_string_length(s:&str)->usize{
    s.len()
}

fn main(){
    let my_String=String::from("Hello, world");
    let length=get_string_length(&my_String);
    println!("{}",length);

   // println!("{}",my_String.chars());

}