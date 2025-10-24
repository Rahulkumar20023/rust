//a function that takes a vector as input and return a vector woth even values


fn main(){
    let vec=vec![1,2,3,4,5];
    println!("len of the vetor is : {}",vec.len());

    let vec2=get_even_vec(&vec);
    println!("vector with even elements : {:?}",vec2);
}


fn get_even_vec(vec:&Vec<i32>)->Vec<i32>{
    let mut vec2=Vec::new();
    for val in vec{
        if val%2==0 {
            vec2.push(*val);
        }
    }
    return vec2;
}
