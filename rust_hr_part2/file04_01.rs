use std::collections::HashMap;

fn main(){
    let mut users=HashMap::new();

    users.insert(0,22);
    users.insert(1, 32);

    let first_user_age=users.get(&0);
    match first_user_age{
        Some(value)=>println!("{}",value),
        None=>println!("This value dont exists"),
    };
}