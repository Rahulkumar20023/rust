#[derive(Debug)]
struct Student{
    name:String,
    age:u8,
    pass:bool,
}

fn main()
{
    let stu=Student{
        name:String::from("Raju"),
        age:8,
        pass:true,
    };
    println!("{:?}",stu);

}