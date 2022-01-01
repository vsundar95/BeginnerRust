mod dcode {

    /* Because functions inside a module is always private, we
     * need to add the pub keyword in front of fn to make it a
     * public function
     */
    pub fn print_message()
    {
        println!("How is it going?");
    }

    /* We can even have a public function call a private function.
     * Here we have the private function chicken and the public
     * function print_chicken which calls the private function chicken
     */
    fn chicken()
    {
        println!("Chicken!!!");
    }

    pub fn print_chicken()
    {
        chicken();
    }

    /* We can even have a module inside of a module */
    pub mod water
    {
        pub fn print_message()
        {
            println!("I am inside the water module!");
        }
    }
}

fn main() {
    dcode::print_message();
    dcode::print_chicken();

    dcode::water::print_message();
}
