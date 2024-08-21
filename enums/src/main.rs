enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64),
}
fn area(shape: &Shape)->f64{
    match shape {
        Shape::Circle(radius    )=>std::f64::consts::PI*radius*radius,
        Shape::Rectangle(len,bred )=>len*bred,
        Shape::Triangle(a, b, c)=>{
            let s=(a+b+c)/2.0;
            (s*(s-a)*(s-b)*(s-c)).sqrt()
        }
    }
}
fn main() {
   let circle=Shape::Circle(4.0);
   let rectangle=Shape::Rectangle(5.0,4.0);
   let triangle=Shape::Triangle(2.0, 3.0, 4.0);
   println!("Circle : {}",area(&circle));
   println!("Rectangle : {}",area(&rectangle));
   println!("Triangle : {}",area(&triangle));
   

}
