enum Discount{
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main()
{
    let n=3;
    match n{
        3=>println!("three"),
        other=>println!("number: {:?}",other),
    }
    //_ ke jagah humne us value ko 
    //ek variable ka naam de diya h, jo 
    //ki other h
    let flat=Discount::Flat(2);

    match flat{
        Discount::Flat(2)=>println!("flat 2"),
        Discount::Flat(amount)=>{
            println!("flat discount of {:?}",amount);
        }
        _=>(),  //do nothing
    };

    let concert=Ticket{
        event:"concert".to_owned(),
        price:50,
    };

    match concert{
        Ticket {price:50,..}=>println!("price @ 50= {:?}",50),//bhai ye price ko match karega, aur .. ka matlab h ki baki jo bhi ho
        //yhi tareeka h struct pe match use karne ka
        Ticket {price,..}=>println!("price = {:?}",price),

    };

}