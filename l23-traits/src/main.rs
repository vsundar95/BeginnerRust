struct Person {
    name: String,
    age : u32
}

impl ToString for Person {
    fn to_string(&self) -> String
    {
        return format!("My name is {}. I am {} years old",self.name, self.age);
    }
}


fn main() {
    let vinny = Person { name: String::from("Vinny"), age: 26 };

    /* ERROR: There is no method named "to_string() in struct Person 
     * println!("{}",vinny.to_string());
     */
    println!("{}",vinny.to_string());
}
