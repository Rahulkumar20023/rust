fn main(){
    let mut vec=vec![1,2,3];

    let ith=vec.iter_mut();

    for val in ith{
        *val=*val+1;
    }
    print!("{:?}",vec);
}