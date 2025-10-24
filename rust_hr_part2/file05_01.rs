fn main()
{
    let nums=vec![1,2,3];
    let iter=nums.iter();

    for value in iter{
        print!("{} ",value);
    }
    println!("{:?}",nums);
}