fn main()
{
    let mut counter=0;

    let mut increase_counter=||{
        counter=counter+1;
        println!("{}",counter);
    };
    //Your closure modifies counter, so it captures &mut counter, which makes it automatically implement FnMut.

    increase_counter();
    increase_counter();
    increase_counter();
}