fn find_first_a(s:String)->Option<i32>{
    for (index,character) in s.chars().enumerate(){
        if character=='a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn main(){
    let my_string=String::from("raman");
    let res:Option<i32>=find_first_a(my_string);

    match res{
        Some(index)=>println!("First a is at index {}",index),
        None=>println!("No a found"),
    }
}