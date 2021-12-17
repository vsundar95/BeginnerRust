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
        println!("Shadowing x: {}",x);

    }
    println!("Original value x: {}",x);

    /* Notice how the value of x does not change now due to the
     * shadowing of the variable x
     */

    /* We can also change data types with shadowing */
    let y = "String y here!";
    println!("y is {}",y);

    let y:bool = true;
    println!("y is {}",y);
}
