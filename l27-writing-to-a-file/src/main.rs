use std::fs::File;
use std::io::prelude::*;

fn main() {
    /* We create a file using the .create() with the name of the file
     * and we use the .expect() incase the file cannot be created
     */
    let mut file = File::create("output.txt")
        .expect("Could not create file");

    /* We can now write to the file using the write_all() function */
    file.write_all(b"Hello, World! I have written to a file!")
        .expect("Cannot write to output.txt");
}
