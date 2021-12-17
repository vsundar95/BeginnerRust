use std::env;


fn main() {

    /* This is how we can ask for command line arguments in Rust.
     * The .args() is used to take in the arguments and the
     * .collect() creates an iterable list of arguments
     */
    let args: Vec<String> = env::args().collect();

    /* We can now iterate over the arguments and I have
     * shown the index of the arguments. Notice that index 0
     * is the name of the rust program
     */
    for (index,argument) in args.iter().enumerate() 
    {
        println!("Argument is {} at index {}",argument, index);
    }
}
