enum Color{
    Red,
    Yellow,
    Blue,
}

fn main()
{
    let color=Color::Yellow;
    match color{
        Color::Blue=>println!("Blue"),
        Color::Yellow=>println!("Yellow"),
        Color::Red=>println!("Red"),
    }
}