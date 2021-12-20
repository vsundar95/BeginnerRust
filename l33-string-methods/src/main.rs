fn main() {
    /* String Replace */
    {
        let my_string = String::from("Rust is fantastic");
        println!("Before replace: {}", my_string);
        println!("After replace: {}",my_string.replace("fantastic", "great"));
    }

    /* String lines */
    {
        let my_string = String::from("The weather is \nnice\noutside mate!");

        /* We are going to iterate over each lines */
        for line in my_string.lines()
        {
            println!("[ {} ]",line);
        }
    }

    /* Split methods */
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}",my_string);
        println!("At index 2: {}",tokens[2]);

    }

    /* Trim methods */
    {
        let my_string = String::from("       Hello, World       \r\n");
        println!("Before Trim: {}",my_string);
        println!("After Trim: {}",my_string.trim());
    }

    /* Chars methods */
    {
        let my_string = String::from("decode on YouTube");

        /* Get character at index */
        match my_string.chars().nth(4)
        {
            Some(c) => println!("Some character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }
}
