fn main() {
    let number=vec![1,2,3,4,5,6,7,8,9,10];
    let result: Vec<i32>=number.iter().filter(|&&x|x%2==0).map(|&x|x*x).collect();
    println!("{:?}",result);
}
