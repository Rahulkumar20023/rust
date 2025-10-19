
fn get_coord()->(i32,i32)
{
    let x=2;
    let y=4;
    (x,y)
}

fn main()
{
    let (x,y)=get_coord();
    if y>5{
        println!("y is greater than 5");
    }
    else if y==5{
        println!("y is equal to 5");
    }
    else{
        println!("y is less than 5");
    }
}