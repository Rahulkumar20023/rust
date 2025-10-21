fn main(){
    let x=1;
    let y=3;
    println!("Sum is {:?}",sum(x,y));
    println!("{:?}",x);
}

fn sum(a:i32,b:i32)->i32{
    a+b
}