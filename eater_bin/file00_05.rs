enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
}
const PI:f64=3.14;
impl Shape{
    fn new_circle(r:f64)->Self
    {
        Shape::Circle(r)
    }
    fn new_rect(l:f64,b:f64)->Self{
        Shape::Rectangle(l,b)
    }
    fn area(&self)->f64{
        let ans=match self{
            Shape::Circle(radius)=>PI*radius*radius,
            Shape::Rectangle(l,b)=>l*b,
        };
        return ans;
    }
}

fn main()
{
    let circle=Shape::new_circle(9.0);
    let rect=Shape::new_rect(5.0,6.0);
    println!("Area of circle : {}",circle.area());
    println!("Area of rectangle : {}",rect.area());
}