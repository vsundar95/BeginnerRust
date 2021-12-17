/* This is an example of a constant.
 * Notice how the constant is defined as an unsigned integer with 8bytes
 */
const MAXIMUM_NUMBER: u8 = 20;

fn main() {
    for n in 1..MAXIMUM_NUMBER
    {
        println!("{}",n);
    }
}
