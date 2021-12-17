fn main() {
    /* Referencing lesson:
     * We have a variable called x which will be 
     * an integer-32. xr will reference that
     * variable.
     */
    let mut x = 10;

    /* xr is called an immutable reference to x
     * this means we cannot change the value of
     * xr even if we add a value to the reference
     */
    let xr = &x;

    /* To fix such issue, we can make a mutable
     * reference and wrap in codeblocks
     */
    {
        let xref = &mut x;
        *xref += 1;
    }

    /* Now we can see that the value of x will 
     * change dueo the reference xref adding a 1
     */
     println!("xref: {}",x);
}
