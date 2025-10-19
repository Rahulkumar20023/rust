struct Temperature{
    degrees_f:f64,
}

impl Temperature {
    fn freezing()->Self{  //ye Self return type hai, matlab Self Temperature h, tu chahe to iss Self ke jagah Temperature bhi likh sakte hai 
        Self {degrees_f:32.0    }
    }
    fn show_temp(&self)
    {
        println!("{:?} F",self.degrees_f);
    }   
}


fn main(){
    let hot=Temperature{degrees_f:98.6};
    //Temperature::show_temp(hot); 
    hot.show_temp();

    let cold=Temperature::freezing();
    cold.show_temp();

}