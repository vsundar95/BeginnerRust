extern crate regex;
use regex::Regex;

fn main() {
    /* Creating a new regex rule */
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "dcod";

    /* Check to see if the string matches the regex */
    println!("Found match?: {}",re.is_match(text));

    /* Creating a capture of the text */
    let regex = Regex::new(r"(\w{5})").unwrap();
    let name = "vinny";

    match re.captures(name)
    {
        Some(caps) => println!("Found a match! {}", caps.get(0).unwrap().as_str()),
        None => println!("Could not find match :(")
    }

}
