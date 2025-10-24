

pub trait Summary{
    fn summarize(&self)->String{
        return String::from("Summarize");
    }
}

struct User{
    name:String,
    age:u32,
}
impl Summary for  User{
    fn summarize(&self)->String{
        return format!("User {} is {} years old",self.name,self.age);
    }
}

fn main()
{
    let user=User{
        name:String::from("Harkirat"),
        age:21,
    };
    println!("{}",user.summarize());
}