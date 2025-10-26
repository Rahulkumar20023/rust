fn main()
{
    let x=String::from("hello");
    let consume_and_return_x=||x;
    //FnOnce mai x ki ownership transfer ho jayega

    let y=consume_and_return_x();
    println!("{:?}",y);

    //let z=consume_and_return_x(); will not work as value of x is moved
}