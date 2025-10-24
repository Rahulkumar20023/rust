//String slices

fn main(){
    let mut word=String::from("Hello World");
    let word2=&word[0..5];

    println!("{}",word2); //Hello
}