fn main() {
    /* This is an example of a tuple of integers */
    let tup1 = (20, 25, 30, 35);

    /* This is how you call a value inside of a tuple */
    println!("{}",tup1.1);

    /* This is an example of a tuple with many values of different types
     * This example also has a nested tuple
     */
    let tup2 = (32, "Vinny", 2.3, true, (1,4,6));
    println!("Printing out a value from the nested tuple of tup2: {}",(tup2.4).1);

    /* We can use tuples to assign values to multiple variables */
    let tup3 = (32, 6.4, "Computer");
    let (a,b,c) = tup3;
    println!("a is {}",a);
    println!("b is {}",b);
    println!("c is {}",c);

}
