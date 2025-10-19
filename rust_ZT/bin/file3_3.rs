enum Flavor{
    Sparkling,
    Sweet,
    Fruity
}


struct Drink{
    flavor:Flavor,
    fluid_oz:f64 
}
fn print_drink(drink: Drink){
    match drink.flavor{
        Flavor::Sparkling=>println!("Drink is sparkling"),
        Flavor::Sweet=>println!("Drink is sweet"),
        Flavor::Fruity=>println!("Drink is fruity")
    }
    println!("oz: {:?}",drink.fluid_oz);
}
fn main()
{
    let soda=Drink{
        flavor:Flavor::Fruity,
        fluid_oz:12.0
    };
    print_drink(soda);
}