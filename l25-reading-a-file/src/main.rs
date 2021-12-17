/* Reading from a file using RUST */
use std::fs::File;
use std::io::prelude::*;

fn main() {
    /* We are opening the file called hello.txt
     * The expect() is used to as a catch in case there is no file to open
     */
    let mut file = File::open("hello.txt").expect("Cannot Open the File");

    /* Dumping the contents of the file into contents */
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Cannot read the file!!");

    println!("Contents are: \n {}",contents);
}
