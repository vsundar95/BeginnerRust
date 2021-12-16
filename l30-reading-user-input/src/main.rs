use std::io;


fn main() {
    /* Store the user input in the string input */
    let mut input = String::new();

    /* User prompted string */
    println!("Input user string: ");

    /* Using match statement to get user input */
    match io::stdin().read_line(&mut input)
    {
        /* Returns success or failure */
        Ok(_) => {
            println!("Success! Response: {}",input);

            /* input is a string that we can do things on */
            println!("Response in uppercase: {}",input.to_uppercase());
        }
        Err(e) => {
            println!("Ooops somthing went wrong! {}",e);
        }
    }


}
