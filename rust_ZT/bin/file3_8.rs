//we can do the same with match expressions to

fn main()
{
    let my_num=3;

    let message=match my_num{
        1=>"hello",
        _=>"goodbye"
    };

    println!("Message is {:?}",message);
}