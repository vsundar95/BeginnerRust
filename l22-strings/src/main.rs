fn main() {
    /* Defining the string here */
    let mut  my_string = String::from("Hello, my name is Vinny. ");

    /* String length */
    println!("Length: {}", my_string.len());

    /* Is Empty */
    println!("String is empty: {}", my_string.is_empty());

    /* Print each item by separating white space */
    for token in my_string.split_whitespace() {
        println!("{}",token);
    }

    /* Contaings a specific word */
    println!("Does the string contain 'Vinny'?: {}", my_string.contains("Vinny"));

    /* We can even push a string onto the existing string. We have to
     * make the existing string mutable by using mut
     */
    my_string.push_str("This is a tutorial about Strings!");
    println!("{}",my_string);
}
