/* Defining a struct */
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    /* Initializing a struct */
    let bg = Color{red: 255, green: 70, blue: 15};
    println!("{}, {}, {}",bg.red, bg.green, bg.blue);

    /* We cannot do this because the initialization of the struct is
     * immutable. We must then call the struct mutable
     */

    let mut fg = Color{red: 255, green: 70, blue: 15};
    println!("Original fg: {}, {}, {}",fg.red, fg.green, fg.blue);
    println!("Now changing the value of blue");
    fg.blue = 32;
    println!("New fg: {}, {}, {}",fg.red, fg.green, fg.blue);
}
