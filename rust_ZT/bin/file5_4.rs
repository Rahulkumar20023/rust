fn main(){
    let my_vec=vec![1,2,2,3,4,5];
    //print!("{:?}",my_vec);
    let mut my_numbers=Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    print!("{:?}",my_numbers);
    my_numbers.pop();
    print!("{:?}",my_numbers.len());

    let two=my_numbers[1];
    print!("{:?}",two);
}