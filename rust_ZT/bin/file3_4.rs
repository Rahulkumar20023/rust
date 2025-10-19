use core::num;

enum Access{
    Full,
}

fn one_two_three()->(i32,i32,i32){
    (1,2,3)
}

fn main(){
    let numbers=one_two_three();
    print!("{:?}",numbers);
    println!("{:?}",numbers.0 );
    println!("{:?}",numbers.1 );
    println!("{:?}",numbers.2 );
    let (x,y,z)=one_two_three();
    println!("{:?}",x);

    let (employee,access)=("Jake",Access::Full);
    print!("{:?}",access);
}