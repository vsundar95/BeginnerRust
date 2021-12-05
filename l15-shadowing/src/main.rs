fn main() {
    /* Shadowing variable defined here */
    let mut x = 10;

    {
        /* If I have this in the code block, the value
         * of x will be 15. Therefore the print statement
         * at the bottom will be 15

        x = 15;
        */
        
        /* Shadowing the first declaration of x */
        let x = 15;
        println!("x: {}",x);

    }
    println!("x: {}",x);
}
