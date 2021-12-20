/* Using the random generator crate */
extern crate rand;

/* Trait part of the rand crate */
use rand::Rng;

fn main() {
    /* Creating a random number from range 1 to 11 excluding 11 */
    let random_number = rand::thread_rng().gen_range(1, 11); 

    println!("Random Number: {}",random_number);

    /* Flipping a coin 
     * The parameter in gen_weighted_bool means I have a 1 in value 
     * chance of being true
     */
    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("Value of random bool is: {}",random_bool);
}
