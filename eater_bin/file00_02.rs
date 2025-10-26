struct Rectangle{
    length:u32,
    breadth:u32,
}

impl Rectangle{
    fn new(new_length:u32,new_breadth:u32)->Self
    {
        Rectangle{
            length:new_length,
            breadth:new_breadth,
        }
    }
    fn area_rec(&self)->u32{
        self.length*self.breadth
    }
}

fn main()
{
    let rec=Rectangle::new(10,5);
}