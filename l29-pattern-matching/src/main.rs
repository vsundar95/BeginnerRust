fn main() {
    /* Integer match statement */
    let number:u32 = 10;

    match number 
    {
        1 => println!("It is one!"),
        /* We can do an inclusive range 2 to 9 */
        2..=9 => println!("It is greater than one!"),

        /* We can add logical statements as well */
        10 | 11 => println!("It is either 10 or 11"),
        _ => println!("It is none of the options")
    }

    /* String match statement */
    let name = "Vinny";

    match name
    {
        "Vinny" => println!("Yes this is Vinny!!"),
        _ => println!("You got the wrong person!")
    }
}
