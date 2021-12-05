struct Color(u8, u8, u8);

fn main() {
    /* This is how we define a tuple struct */
    let red = Color(255, 0, 0);
    println!("red is {}, {}, {}",red.0, red.1, red.2);

    /* If we wanted to change the value in a particular index in red,
     * we need to make the tuple struct mutable
     */
    let mut redMut = Color(255,  0, 0);
    redMut.2 = 45;
    println!("redMut is {}, {}, {}", redMut.0, redMut.1, redMut.2);

}
