struct GroceryItem{
    stock: i32,
    price: f64,
}

fn main()
{
    let cereal=GroceryItem{
        stock:10,
        price:3.99,
    };
    println!("stock: {:?}",cereal.stock);
    println!("price: {:?}",cereal.price);
}