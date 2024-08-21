use std::fmt::Error;

fn divide(a: f64, b:f64)-> Result<f64, &'static str>{
    if b==0.0{
        Err("can not divide by zero")
    }else{
    Ok(a/b)
}
}
fn main() {
   match  divide(4.0, 6.0) {

       Ok(result)=>println!("{}",result),
       Err(e)=>println!("{}",e),
   }
   match divide(8.0, 0.0) {
    Ok(result)=>println!("{}",result),
    Err(e)=>println!("{}",e),
   }
}
