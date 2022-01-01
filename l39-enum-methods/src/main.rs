// This is to allow dead code
#![allow(dead_code)]

enum Day
{
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}
/* Just like structs, we can use an implementation method to have
 * functions work on the enum
 */
impl Day 
{
    pub fn is_weekday(&self) -> bool
    {
        match self {
            // This is a reference to the enum Day
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
        }
    }
}
fn main() {
    let day = Day::Saturday;
    println!("Is day a weekeday? {}", day.is_weekday());
}
