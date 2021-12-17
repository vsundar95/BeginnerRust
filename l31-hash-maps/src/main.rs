use std::collections::HashMap;

fn main() {
    /* Creating a hash map */
    let mut marks = HashMap::new();

    /* Adding values to the hash map 
     * Key,Value
     */
    marks.insert("Rust Programming", 96);
    marks.insert("Mathematics", 92);
    marks.insert("English", 87);
    marks.insert("History", 91);

    /* Find the length of the HashMap */
    println!("How many subjects have you taken? {}",
             marks.len());

    /* Use the get method of hash map to get a particular value 
     * To get this value we need to use a match statement
     */
    match marks.get("Rust Programming")
    {
        /* Use the some() to check to see if there is a key called
         * Rust Programming and if there is we return the grade
         */
        Some(mark) => println!("You got {} for Rust Programming",mark),

        /* If there is not a class called Rust Programming then
         * we throw out a statement saying we cannot find it
         */
        None => println!("Cannot find grade for Rust Programming")
    } 

    /* Removing a value using the remove method */
    marks.remove("English");

    /* Looping through Hash Map using for-in loop */
    for (subject, mark) in &marks
    {
        println!("For {} you got {}%",subject, mark);
    }

    /* We can check to see if a key exists in a map */
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));
    
}
