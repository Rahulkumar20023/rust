enum Direction{
    North,
    East,
    South,
    West,
}

fn main(){
    let my_direction=Direction::North;
    let new_direction=my_direction; //No error, becuase Direction is copy
    move_around(new_direction);
}

fn move_around(direction::Direction){
    //implements logic to move a chara toer around
}