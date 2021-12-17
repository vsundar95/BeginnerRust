/* Creating a struct called Rectangle for impl demo */
struct Rectangle{
    width: u32,
    length: u32
}

/* This is our impl function that will be based off the struct
 * Rectangle. We pass in the &self to reference the struct
 * Rectangle.
 */

impl Rectangle{
    fn print_description(&self){
        println!("Rectangle: {} x {}", self.width, self.length);
    }

    fn calculate_area(&self){
        let area: u32 = self.width * self.length;
        println!("Area of Rectangle is {}",area);
    }

    fn is_square(&self) -> bool{
        self.width == self.length
    }
}

fn main() {
    let my_rect = Rectangle { width: 5, length: 3 };
    my_rect.print_description();
    my_rect.calculate_area();
    println!("My Rectangle is a square: {}",my_rect.is_square());  
}
