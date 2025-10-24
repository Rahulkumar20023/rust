fn fib(n:i32)->i32{
    let mut a=0;
    let mut b=1;
    

    for _ in 2..n{
       let c=a+b;
       a=b;
       b=c; 
        
    }
    return b;
}

fn main()
{
    print!("{}",fib(7));
}