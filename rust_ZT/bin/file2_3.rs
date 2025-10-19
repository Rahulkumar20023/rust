
fn sum(a:i32,b:i32)->i32{
    a+b
}

fn display(c:i32){
    println!("{:?}",c);
}
fn main()
{
    let a=3;
    let b=4;
    let c=sum(a,b);
    display(c);
}