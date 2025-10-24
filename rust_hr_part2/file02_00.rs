enum CustomOption{
    Some(i32),
    None,
}

fn main(){
    let index=find_first_a(String::from("preet"));

    match index{
        CustomOption::Some(value)=>println!("Index is {}",value),
        CustomOption::None=>print!("a not found"),
    }
}

fn find_first_a(s:String)->CustomOption {
    for (index,ch) in s.chars().enumerate(){
        if ch=='a'{
            return CustomOption::Some(index as i32);
        }
    }

    return CustomOption::None;
}