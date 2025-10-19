struct Temperature{
    degrees_f:f64,
}

fn show_temp(temp: Temperature)
{
    println!("{:?} F",temp.degrees_f);
}

fn main(){
    let hot=Temperature{degrees_f:98.6};
    show_temp(hot); 
}