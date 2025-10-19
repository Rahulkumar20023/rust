struct Persons{
    age:i32,
    name:String,
    color:String,
}

fn main()
{
    let vector=vec![
        Persons{age:23,name:"Alice".to_owned(),color:"Red".to_owned()},
        Persons{age:3,name:"Bob".to_owned(),color:"Blue".to_owned()},
        Persons{age:25,name:"Charlie".to_owned(),color:"Green".to_owned()},
    ];

    for item in vector{
        if item.age<10{
            println!("name: {:?}, age: {:?}, color: {:?}",item.name,item.age,item.color);
        }
    }
}