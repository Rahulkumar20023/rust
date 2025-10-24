fn main()
{
    let v1=vec![1,2,3];
    let iter=v1.iter();
    let iter2=iter.map(|x| x+1);
    for x in iter2{
        println!("{}",x);
    }
}