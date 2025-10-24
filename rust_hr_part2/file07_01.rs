

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
    //jis struct ne Sumamry ko overwrite kar diya to uske liye 
    //wo function chalega par agar overwrite ni kiay h to 
    //jo Summary implment kiya hu wo chalega
    notify(user);

}

fn notify(u:impl Summary)
{
    println!("{}",u.summarize()); //dekh jaise u i32 sab ho sakta that
    //to yaha bhi u ka type prestricuted h
    //u wo sab ho sakta h jisne Summary ko impl kiya ho
    //eg: impl Summary for User
}