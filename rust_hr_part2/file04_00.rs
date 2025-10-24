use std::collections::HashMap;

fn main(){
    let mut users=HashMap::new();

    users.insert(String::from("harkirat"),22);
    users.insert(String::from("raman"), 32);

    let first_user_age=users.get("harkirat");
    match first_user_age{
        Some(value)=>println!("{}",*value),
        None=>println!("This value dont exists"),
    };
}