fn main() {
   let name = String::from("Vinny");

   println!("Character at index 1: {}", match name.chars().nth(1){
       Some(c) => c.to_string(), //storing the character in variable c
       None => "No Character at index 8".to_string()
   });

   /* Now calling out the get_occupation function */
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
