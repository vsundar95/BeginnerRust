fn main() {
    /* This is an example of a variable in RUST
     * Notice how we don't give it a type of integer
     * RUST can already identify that the type is integer
     */
    
    // Using the let command means we are instantiating the variable x
    let x = 45;
    println!("The value of x is {}\n", x);
    println!("This x is an immutable variable meaning the value cannot change\n");
    
    /* Now initializing x = 60 will throw out an error
     *
     * x = 60;
     * println!("The new value of x is {}\n", x);
     * 
     * Main reason for this is we are using the let command to create the variable x
     * In RUST, all variables are immutable. To make an immutable variable, we need to
     * use the mut command.
     *
     * let mut x = 60;
     * println!("The new value of x is {}\n", x);
    */ 
    let mut x = 60;
    println!("The old value of x is {}\n",x);
    x = 32;
    println!("The new value of x is {}\n",x);
    println!("This x is a mutable variable meaning the value cannot change\n");

}
