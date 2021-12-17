fn main() {
    /* Standard for-loop example */
    for i in 1..11
    {
        println!("The value of i is {}",i);
    }
    println!("\n");

    /* We can also have variables be iterators as well */
    let numbers = 30..51;

    for i in numbers
    {
        println!("The value of i is {}",i);
    }
    println!("\n");

    /* We can also loop over a vector */
    let animals = vec!["Rabbit", "Dog", "Cat"];
    
    /* When using vectors, use the .iter() to iterate over the vector */
    for a in animals.iter()
    {
        println!("The animal name is {}",a);
    } 

    println!("\n");

    /* We can even find the index of the value inside the vector */
    for (index, a) in animals.iter().enumerate()
    {
        println!("The index {} is animal name {}",index,a);
    }   
}
