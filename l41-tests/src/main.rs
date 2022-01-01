struct Rectangle
{
    width: u8,
    height: u8
}

impl Rectangle
{
    fn is_square(&self) -> bool
    {
        self.width == self.height
    }
}

fn main() {
    println!("Hello, world!");
}

fn give_two() -> u32
{
    2
}
/* The below line prevents the test from being compiled at run time when
 * the user runs cargo run.
 * This test will only be run when the user runs cargo test
 */
#[cfg(test)]
mod dcode_tests
{
    /* Telling cargo that the function following the line below is a test */
    #[test]

    /* If you expect the test to fail, we need to use ths at the bottom */
    #[should_panic]
    fn test_basic()
    {
        /* Asserting that something is true */
        assert!(1 == 1);

        /* Making this test fail */
        panic!("Oh no!");
    }

    #[test]
    /* To ignore a test, we use the below */
    //#[ignore]
    fn test_equals()
    {
        assert_eq!(2, 1+1);
        assert_ne!(2, 2+1);

        /* Use the super to call out of scope functions */
        assert_eq!(super::give_two(), 1+1);
        assert_ne!(super::give_two(), 2+1);
    }

    #[test]
    fn test_structs()
    {
        let r = super::Rectangle {
            width: 50,
            height: 50
        };

        assert!(r.is_square());
    }
}
