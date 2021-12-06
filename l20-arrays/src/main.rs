fn main() {
    /* Defining a simple number array */
    let numbers = [1,2,3,4,5];

    /* Printing out the value in the 0th index */
    println!("first value is: {}",numbers[0]);

    /* We can even print using a for-loop using
     * the .iter() command
     */
    for num in numbers.iter()
    {
        println!("{}",num);
    }

    println!("-----> ---------- <-----");
    /* We can print using the indices of the numbers
     * array as well
     */
    for index in 0..numbers.len()
    {
        println!("{}",numbers[index]);
    }

    /* This array has the value 2 in every position
     * with size of 20
     */
    println!("-----> ---------- <-----");

    let two_array = [2; 20];
    
    for (index,val) in two_array.iter().enumerate()
    {
        println!("{} --> {}",val,index);
    }

}
