enum Direction{
    Left,
    Right
}

fn main(){
    let go=Direction::Left;
    match go{
        Direction::Left=>println!("Going Left"),
        Direction::Right=>println!("Going Right"),
    }
}