fn main() {
    let x: u32 = 10;

    /* Defining the code block here */
    {
        let y: u32 = 5;
        /* The print statement has access to the value of
         * x and y so this println! statement is valid
         */
        println!("x: {}, y: {}",x,y);
    }

    /* This print statement has access to x but not y because it is not inside
     * the code block {}
     * println!("x: {}, y: {}",x,y);
     */
}
