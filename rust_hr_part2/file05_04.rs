fn main()
{
    let v1=vec![1,2,3];
    //let mut v1=vec![1,2,3];
    let v1_iter=v1.into_iter();

    for val in v1_iter{
        println!("{}",val);
    }

    //println!("{:?}",v1); //into_iter se ownership chale jayegi v1 ki
}