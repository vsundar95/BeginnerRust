fn main() {
    /* First way of creating a vector.
     * This creates a blank vector without the need
     * for inserting elements
     */
    let mut test_vector_1: Vec<u32> = Vec::new();

    /* Because the vector is empty, we are going to add u32 integers
     * inside the vector. We will be using the push command. NOTE:
     * The vector needs to be mutable so add the mut keyword
     */
    for num in 1..10
    {
        test_vector_1.push(num);
    }

    /* We use the remove command to remove a value in the position */
    test_vector_1.remove(3);

    for num in test_vector_1.iter()
    {
        println!("{}",num);
    }

    println!("-----> ---------- <-----");
    /* Second way of creating a vector
     * This is similar to that of an array
     */
    let mut test_vector_2 = vec![0,1,2,3,4];
    test_vector_2.push(5);
    test_vector_2.remove(2);

    for num in test_vector_2.iter()
    {
        println!("{}",num);
    }

}
