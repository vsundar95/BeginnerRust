fn main() {
    print_numbers_to(31);
}

fn print_numbers_to(num: u32)
{
    for n in 1..num
    {
        if is_even(n)
        {
            println!("{} is even",n);
        }
        else
        {
            println!("{} is odd",n);
        }
    }
}

/* This function takes in a number and returns a boolean */
fn is_even(num: u32) -> bool
{
    /* If the number is even return true, else return false */
    return num % 2 == 0;
}
