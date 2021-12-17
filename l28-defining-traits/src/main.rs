struct Person {
    name: String,
    age: u8
}

/* Defining the traits for the functions to be performed on Person struct */
trait HasVoiceBox {
    fn speak(&self); /* Let the person speak? */

    fn can_speak(&self) -> bool; /* Can the person speak? */
}

/* Now we define the traits for the Person struct */
impl HasVoiceBox for Person {
    fn speak(&self)
    {
        println!("Hello! My name is {}",self.name);
    }

    fn can_speak(&self) -> bool
    {
        if self.age > 0
        {
            return true;
        }
        return false;
    }

}
fn main() {
   /* Initialize the Person struct */
   let person = Person { 
       name: String::from("Vinny"),
       age: 26
   };
   /* Call the speak function from the trait */
   person.speak();

   /* Call the can_speak function from the trait */
   println!("Can {} speak? {}",person.name, person.can_speak());
}
