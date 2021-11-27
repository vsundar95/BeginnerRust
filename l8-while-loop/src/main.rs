fn main() {
    let mut n = 1;

    /* Creating a while loop */
    while n <= 50
    {
        /* Check to see if n is a multiple of 5 by using % modulo sign */
        if n % 5 == 0
        {
            println!("n is {}",n);
        }

        n += 1; 

    }
}
