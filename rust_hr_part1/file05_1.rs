//enums with values asscoicted with them''

enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn main(){
    let circle=Shape::Circle(5.0);
    let square=Shape::Square(4.0);
    let rectangle=Shape::Rectangle(3.0,6.0);
}