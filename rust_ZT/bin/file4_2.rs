struct Grocery{
    quantity: i32,
    id:i32,
}

fn display_quantity(item: &Grocery)
{
    println!("quantity: {:?}",item.quantity);
}   
fn display_id(item: &Grocery)
{
    println!("id: {:?}",item.id);
}
fn main()
{
    let item=Grocery{
        quantity:5,
        id:101,
    };
    display_quantity(&item);
    display_id(&item);

}