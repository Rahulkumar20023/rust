struct Adult{
    age:u8,
    name:String,
}

impl Adult{
    fn new(age: u8, name: &str)->Result<Self,&str>{
        if age>=21 {
            Ok(Self { age, name: name.to_string() }),
        }else{
            Err("Not an adult")
        }
    }
}


fn main(){
    let adult=Adult::new(25,"Jayson");
    match adult{
        Ok(a)=>println!("Adult created: {:?}",a.name),
        Err(e)=>println!("Error: {:?}",e),
    }
}
//5:35:22