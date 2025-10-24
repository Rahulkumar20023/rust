fn main()
{
    let mut v1=vec![1,2,3];

    let mut v1_iter=v1.iter();

    while let Some(val)=v1_iter.next() {
        println!("{}",val);
    }

    println!("{:?}",v1);
}