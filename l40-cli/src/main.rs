use std::process::Command;

fn main() {
    // We want to pass in the command to run in terminal
    // python3 hello.py

    // Creating the Command block
    let hello = Command::new("/usr/bin/python3")
        .arg("hello.py")
        .output()
        .expect("Failed to run python file!");

    // Printing out the output from the python3 command
    println!("Output: {}", String::from_utf8_lossy(&hello.stdout));
}
