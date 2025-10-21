enum Directions{
    North,
    East,
    South,
    West,   
}

fn main(){
    let my_direction=Directions::South;
    //let new_direction=my_direction;//No error, becuse Direction is Copy
    move_Around(my_direction);
    match my_direction{
        Directions::North=>println!("Direction is North"),
        Directions::East=>println!("Direction is East"),
        Directions::South=>println!("Direction is South"),
        Directions::West=>println!("Direction is West"),
    }
}
fn move_Around(dir:Directions){
    match dir{
        Directions::North=>println!("Moving North"),
        Directions::East=>println!("Moving East"),
        Directions::South=>println!("Moving South"),
        Directions::West=>println!("Moving West"),
    }
}