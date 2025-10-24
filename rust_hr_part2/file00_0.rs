fn is_even(n:i32)->bool{
    return n%2==0;
}


fn main()
{
    let n=4;
    if is_even(n){
        println!("even");
    }
    else{
        println!("odd");
    }
}