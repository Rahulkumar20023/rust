fn main()
{
    let mut s1=String::from("Hello");
    let s2=&s1;
    println!("s2 is {}",s2);
    let s3=&mut s1;
    s3.push_str(" World");
    println!("s3 is {}",s3);
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);
    
}

