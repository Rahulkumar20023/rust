struct Counter{
    count:u32,
}

impl Counter{
    fn new()->Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter{

    type Item=u32;

    fn next(&mut self)->Option<Self::Item> {
        self.count+=1;
        if self.count<=5 {
            Some(self.count)
        }
        else{
            None
        }
    }
}

fn main()
{
    let counter=Counter::new();
    for num in counter{
        println!("{}",num);
    }

    //let mut iter2=counter.into_iter();

    // loop{
    //     match iter2.next(){
    //         Some(val)=>println!("Count: {:?}",val),
    //         None=>break,
    //     }
    // }

    let mut iter=Counter::new();

    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());

}