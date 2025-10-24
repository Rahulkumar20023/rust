fn main(){
    let nums=vec![1,2,3];
    for value in &nums{
        print!("{} ",*value);
    }
    //println!("{:?}",nums);
}