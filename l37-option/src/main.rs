fn main() {
   let name = String::from("Vinny");

   println!("Character at index 1: {}", match name.chars().nth(1){
       Some(c) => c.to_string(), //storing the character in variable c
       None => "No Character at index 8".to_string()
   });

   /* Now calling out the get_occupation function
    * With this we can get the return value from the get_occupation
    * function and use that as an option. Notice how we are returning
    * either a Some or a None and that value is being printed out
    * in this println! 
    */
   println!("Occupation is {}", match get_occupation("Vinny") {
       Some(o) => o,
       None => "There is no occupation"
   });
}

/* We are going to define an option return type */
fn get_occupation(name: &str) -> Option<&str>
{
    match name
    {
        "Vinny" => Some("Software Developer"),
        "Aditya" => Some("UF Student"),
        _ => None
    }
}
