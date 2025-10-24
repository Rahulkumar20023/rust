fn main(){
    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    even_values(vec);
    //println!("Updated vector is : {:?}",vec);
}

fn even_values(mut v: Vec<i32>)
{
    let mut i=0;
    while i<v.len() {
        if v[i]%2!=0 {
            v.remove(i);
        }
        else{
            i=i+1;
        }
    }
}