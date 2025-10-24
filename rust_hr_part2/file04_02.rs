use std::{collections::HashMap, hash::Hash};

fn main(){
    let pairs=vec![
        (String::from("harkirat"),21),
        (String::from("raman"),31),
        (String::from("harkirat"),15)
    ];

    let grouped_pairs=grouped_pairs(&pairs);

    for (key,value) in grouped_pairs {
        println!("{}: {:?}",key,value);
    }
}

fn grouped_pairs(vec: &Vec<(String,i32)>)->HashMap<String,Vec<i32>>
{
    let len=vec.len();
    let mut hash:HashMap<String, Vec<i32>>=HashMap::new();
    for i in 0..len{
        let val=&vec[i];
        let (key,value)=val;
        let mut hash_val=hash.get_mut(key);
        match hash_val{
            Some(v)=>{
                v.push(*value);
            },
            None=>{
                let mut vect=Vec::new();
                vect.push(*value);
                hash.insert(key.clone(),vect);
            }
        }
    }
    return hash;
}