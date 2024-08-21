enum Direction {
    East,
    West,
    North,
    South
}
fn get_dir(direction:Direction)-> &'static str{
    match direction {
        Direction::East=>"East",
        Direction::North=>"North",
         Direction::South=>"South",
         Direction::West=>"West"
    }

}
fn main() {
    let direction=Direction::South;
    println!("Direction :{}",get_dir(direction));
 
}