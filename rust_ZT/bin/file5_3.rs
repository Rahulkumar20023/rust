enum Color{
    Brown,
    Red
}

impl Color{
    fn print(&self){
        match self{
            Color::Brown=>println!("Color is Brown"),
            Color::Red=>println!("Color is Red"),
        }
    }
}

struct Dimensions{
    width:f64,
    height:f64,
    depth:f64,
}

impl Dimensions{
    fn print(&self){
        println!("Width: {:?}, Height: {:?}, Depth: {:?}",self.width,self.height,self.depth);
    }
}

struct ShippingBox{
    color:Color,
    weight:f64,
    dimensions:Dimensions,
}

impl ShippingBox{
    fn new(weight:f64, color: Color, dimensions:Dimensions)->Self{
        Self {weight,color,dimensions}
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}",self.weight);
    }
}

fn main(){
    let dim=Dimensions{width:10.0,height:20.0,depth:30.0};
    let box1=ShippingBox::new(50.0,Color::Brown,dim);
    box1.print();
}


