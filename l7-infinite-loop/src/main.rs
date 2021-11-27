fn main() {
     let mut n = 0;
    
     /* Starting the loop now */
     loop 
     {
         n += 1; // n = n + 1
         
         /* If we leave it at this, we can cause an infinite loop 
          * So lets add some conditions!
          */

         if n == 7 
         {
             /* We are going to skip the value of 7 and have the loop continue */
             continue;
         }
         else if n > 10
         {
             /* Stop the loop if n is greater than 10 */
             break;
         }
         else
         {
             println!("The value of n is {}",n);
         }
    }
}
