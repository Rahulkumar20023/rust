fn main()
{
    let v1=vec![1,2,3];
    let iter=v1.iter();
    let iter2=iter.filter(|x| *x % 2==0);
    for x in iter2 {
        println!("{}",x);
    }
}