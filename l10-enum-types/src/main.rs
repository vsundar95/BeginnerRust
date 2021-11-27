/* Defining the enumeration (enum) type before the main function */
enum Direction
{
    Up, 
    Down,
    Left,
    Right
}

fn main() {
    /* We can define the Enum Direction */
    let player_direction:Direction = Direction::Left;

    /* Treat the match operator as a switch statement depending on the value of player_direction */
    match player_direction
    {
        Direction::Up => println!("The direction is Up"),
        Direction::Down => println!("The direction is Down"),
        Direction::Left => println!("The direction is Left"),
        Direction::Right => println!("The direction is Right"),
    }
}
