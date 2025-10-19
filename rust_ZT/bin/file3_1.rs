enum Colors{
    Red,
    Green,
    Blue
}

fn print_color(my_color:Colors){
    match my_color{
        Colors::Red=>println!("Color is Red"),
        Colors::Green=>println!("Color is Green"),
        Colors::Blue=>println!("Color is Blue"),
    }
}
fn main(){
    let color=Colors::Green;
    print_color(color);
}