/* Struct is going to be passed by reference */
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let blue = Color { red: 0, green: 0, blue: 255 };
    
    /* This is how you pass a struct by reference by using
     * the & sign
     */
    print_color(&blue);
}

/* Passing the struct Color by reference
 * This is how you  pass a struct by reference
 */
fn print_color(c: &Color)
{
    println!("Color - R: {}, G: {}, B: {}",c.red, c.green, c.blue);
}
