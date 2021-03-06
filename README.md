# BeginnerRust
https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL

## Installation Guide 
https://www.rust-lang.org/lear/get-started

I first ran the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

I also ran `rustup update` to make sure there are any updates

### First Steps with Cargo
To start a new package with Cargo, use `cargo new <package name>`
Use the `tree <package name>` to see the tree of files

The file Cargo.toml is called a manifest that contains all the metadata that Cargo needs to compile the package

To run the src/main.rs file we need to use `cargo run` which will compile and run all in one step

The Cargo.lock contains information about our dependencies

Once we are ready for the release, we use `cargo build --release`. This command puts the binary in target/release instead of target/debug. compiling in debug mode is the default for develoopment. Release mode takes longer to compile but code will run faster.

#### Lesson 1-2 Hello World
Crate Name : hello_world

#### Lesson 3 Comments
Crate Name : hello_world

Comments are exactly the same in C where `//` means one line of comment and `/* */` means a block of comments

#### Lesson 4 Variables
Crate Name : l4-variables

To create an immutable variable in RUST, we use the `let` command. For example:
    `let x = 60;`
Notice how we did not add a type like integer. RUST will automatically determine the type of the variable. 

Now if we were going to change the value of x to 45, the RUST compiler will throw an error. This is because using the `let` command creates an immutable variable. This means that the value of the variable `x` cannot change. 

To make an mutable variable, we need to use the `mut` command
    `let mut x = 60;`

#### Lesson 5 Variable Data Types
Crate Name : l5-variable-data-types

RUST immediately assumes the data type of the variable. When we initialize `let x = 45`, RUST says that this variable is a data type of integer-32. However, we can specify the data type by putting a `:` and the data type. For example: `let x: i64 = 45` making it an integer-64. There are a lot of data types that can be used in RUST. Please check the sample code for more information. 

#### Lesson 6 If Else Statements
Crate Name : l6-if-else-statements

RUST uses a similar syntax when crating if-else if-else statements. RUST uses `{}` for the conditionals. NOTE: RUST will throw out a warning when the developer uses `()` to contain the condition. Therefore `()` are not needed for conditions.

#### Lesson 7 Infinite Loop
Crate Name : l7-infinite-loop

To begin a loop, we use the `loop` command. To exit out of a loop we use `break` and to continue without doing anything to the current value, we use the `continue` command. Nothing too complex or different from C/C++

#### Lesson 8 While Loop
Crate Name : l8-while-loop

Exactly like C/C++ with while loop just without the parenthesis. Syntax is `while <condition> { execute commands ; increase counter }`

#### Lesson 9 For loop
Crate Name : l9-for-loop

All for-loop must use an iterator. An iterator is defined by the `..` syntax. For example, `1..11` means 1 to 10 excluding 11. This is a range from 1 to 11 non-exclusive the upper bound. 
The syntax for a for loop is like python's for-loop: `for i in 1..11 { condition }`. 

We can even have variables be set to iterators as well. This is set as `let numbers = 31..50` which will create a list of numbers from 31 to 50. Simply call the for-loop as `for i in numbers { condition }`.  

A for-loop can also be used to iterate over the vector as well. First we need to create a vector using the `vec!` command -- vec~[A, B, C, ....] and this vector can be strings or integers depending on the values given. Now for iterating on a vector, we use the .iter() command. For example, `for a in <vector>.iter() { condition }`. 

We can even get the index of a value inside the vector using the .enumerate() command. For example: `for (index, val) in vector.iter().enumerate() { condition }`

#### Lesson 10 Enum Types
Crate Name : l10-enum-types

Enums are a way to express your code in a simple and descriptive way. We have to declare the enum before the main function. Enums are defined by `enum <Name> { enum values }`

To call an enum type we simply let a variable be defined to the enum. For example, `let variable:<enum Name> = <enum Name>::<default enum Value>;`

Now if we want to do some control flow using the enum, we can use the `match` operator. This allows the developer to compare a value against a series of patterns and then execute code based on which pattern matches. It is like a switch command in C/C++. 

#### Lesson 11 Constants
Crate Name : l11-constants

Constants are variables that are declared in the global scope and cannot be changed.
We use the `const` keyword followed by the name of the const. With constants, we
need to specify the type of data. 

#### Lesson 12 Tuples
Crate name : l12-tuples

Tuples are a bunch of variables in a collection. They are defined as such:
`let tup1 = (20, 25, 30, 35);` and are comma separated. To print out a value
in a specific index of the tuple you write `tup1.<index>`. We can even store values
of different types like int, boolean, floats, strings. We can even have a nested
tuple. To access the nested tuple, we need to wrap the original tuple in () and then
we can write the index.  

Another use for tuples is for declaring mutliple variables at once. We simply
create another tuple of variables and assign it to the assigning tuple. 

#### Lesson 13 Functions
Crate name : l13-functions

Functions are always defined by the `fn` keyword. The parameters of the functions
need to be defined. So if the program is passing in an integer, we then have to
set the parameter to u32 or unsigned int-32. 

Functions can also return a result. Depending on the type of the result we use the
`->` to return the type of the result. 

#### Lesson 14 Code Blocks
Crate name : l14-code-blocks

A code block is a piece of code that is found inside two { }. It is oscilated
but does have access to data variables defined outside { }. However, the 
outside { } does not have access to the variables inside the code block { }. 
The variables in { } exist only inside { }. 

#### Lesson 15 Shadowing
Crate name : l15-shadowing

Shadowing is the concept of resuing a variable name rather than creating two
unique variables. We use code blocks to shadow a variable. A simple 
example is:
`let x = 5; let x = x + 1; let x = x * 2;`
 
We can see that the value of x changes due to the second statement from 6 to
12 because the first statement is being "shadowed" by the second statement. 

We can even use mutable types to show shadowing simply by using the `mut` 
prefix.

#### Lesson 16 - References
Crate name : l16-references

References are a way to call a variable with a different name. We are referecing
an original varialbe. This is similar to c++ with pointers. We use the `&` to
reference. 

`let x = 10; let xr = &x;`

We can see that xr is referencing x. 

To change the value of the original value using a reference, we need to 
wrap the reference in codeblocks, change the reference to a mutable reference
`&mut` and perform any update to the original value.

#### Lesson 17 - Structs
Crate name : l17-structs

To define a struct, we simply call `struct` outside the main function. The
variables in the struct need to be defined with the data type (u8, u32, ..).

To initialize a struct, use the let command and call the struct using { }. 
Inside the { } we have the variables defined. 

To change the value inside a struct, we need to define the initialized struct
as a mutable struct `mut`. 

#### Lesson 18 - Tuple Structs
Crate name : l18-tuple-structs

Tuple structs are almost identical to regular structs except they are treated
as tuples. Simply create a tuple struct before the main function by 
using ( ) instead of { }.

The only difference is that tuples use index values instead of variable names.

#### Lesson 19 - Pass by Reference
Crate name : l19-pass-by-reference

For this example, we are passing a struct by reference. We would have to use the
& sign to pass it to a function. Now we can remove the & and pass the struct
without using the & sign but doing so will remove the struct out of scope. 
So to avoid any out of scope issue, we use the & and pass by reference.   

#### Lesson 20 - Arrays
Crate name : l20-arrays

A sequence of stuff that you can access.
`let numbers = [1,2,3,4,5];`

To access a value in an array, call the array but put the position inside the
`[]`. So `numbers[0]`. To iterate through an array, use the `.iter()` command.
You can also loop using indices as well starting from 0 and ending in 
array.len().

Arrays can also be data typed. 
`let numbers: [i32; 5] = [1,2,3,4,5]` which means we have defined an array
of type integer-32 with a defined size of 5.

We can even have an array of a single value n number of times:
`let numbers = [2; 400];` which means array size 400 has the value 2 in all
elements.  

#### Lesson 21 - Impl (Implementation) Keyword
Crate name : l21-impl

This is a way to add methods to a struct to make it useful making the struct
an object. These impl functions will only work on the struct since we used
the name of the struct. Impl functions start with the word `impl`. 

#### Lesson 22 - Strings
Crate name : l22-strings

There are two types of strings: primitive and String data type. We will be
defining the String data type. We use the `String::from("STRING");`

We can perform more functions on the string data type as seen in the cargo file.

Documentation: https://doc.rust-lang.org/std/string/struct.String.html

#### Lesson 23 - Traits
Crate name : l23-traits

A trait is an interface that something an object i.e. struct can do. It is
similar to that of the impl function but it is a specific trait. Traits define shared
behavior in an abstract way. 

The example uses the to_string trait where we need to create an impl function
and "for" it from the struct. 

#### Lesson 24 - Vectors
Crate name : l24-vectors

Vectors are like arrays but can be dynamic. We can define vectors in two 
ways:
`Vec<u32> = Vec::new()` or using `vec![]` macro. 

#### Lesson 25 - Reading a File
Crate name : l25-reading-a-file

Reading from a file is simple with rust. First we need to import two modules
at the top of the program:
`use std::fs::File;` and `usr std::io::prelude::*;`

Now we must open the file using `File::open`. Notice that there is a `.expect`
at the end of the open command. This is used to throw an error when the file
cannot open. 

We can create a string and put the contents of the file into the string. This
is done using the `read_to_string` function as well as an `expect` trail. 

#### Lesson 26 - Command Line Arguments
Crate name : l26-command-line-arguments

We can take in command line arguments from rust. This is similar to that of C/C++.
First we need to add a package at the top: `use std::env`

Now we use the command `env::args().collect()` to get the command line arguments
and make them iterable. 

#### Lesson 27 - Writing to a File
Create name : l27-writing-to-a-file

Using the same header as reading from a file, we can also write to a file.
This process is straightforwad. We simple need to create the file and write to the file.
To create a file, we need to use the `File::create()` function to create the file
by entering the file name. We also use the `write_all()` function to write
text to the file. Just note when writing to the file use the `b""` when starting
your text. This will convert the string to bytes and place it into the file. 

#### Lesson 28 - Defining Traits
Crate name : l28-defining-traits

A trait is a certain set of rules that an object must have in order to have
the name of the trait. These traits are particular functions that can be used
as impl functions in Rust. 

#### Lesson 29 - Pattern Matching
Crate name : l29-pattern-matching

A `match` statement is like a case and switch statement in C++. The switches
are based on the values of the match variable. For the default, we use the `_`
instead of saying default. For example:

```
let number = 2;
match number
{
	1 => condition
	2 => condition
	_ => condition
}
```
This is the basics of a match statement. Match statements can also be used
for strings as well. NOTE: Do not initialize a string as String::from("") because
it will throw a struct error. So simply initialize it as "String". 

#### Lesson 30 - Reading User Input
Crate name : l30-reading-user-input

We can use `io::stdin().read_line()` to read in user data. This comes from
the `use std::io` package at the top of the program. User data can be put into
a match statement where we match on OK(_) where that matches on default and 
Err(e) which matches on an error. 

#### Lesson 31 - Hash Maps
Crate name : l31-hash-maps

Hash map is a collection of key value pairs. Keys are mapped to values. We need
to import the hash map struct into Rust using `use std::collections::HashMap`

In this example we talk about inserting, getting, printing all key and value pair, removing a key and value pair,
and checking to see if a key exits.

#### Lesson 32 - Random Numbers 
Crate name : l32-random-numbers

First we need to import the rand crate in the Cargo.toml. Under dependencies, we
need to add `rand = "0.3"`. Now in the .rs file, we need to add at the top,
`extern crate rand` and `use rand::Rng`.

For integer values, we need to generate a range using the `.gen_range(min, max)`
command. For boolean values, we need to use the `.gen_weighted_bool(x)` command
where x represents the value of 1 in x values being true.  

#### Lesson 33 - String Methods
Crate name : l33-string-methods

For this tutorial on string methods, we chose to cover five main methods for Strings:
	-> replace
	-> lines
	-> split
	-> trim
	-> chars

#### Lesson 34 - Multiple Source Files
Crate name : l34-multiple-source-files

When creating multiple source files we need to include them as modules. TO
do this, we add the `mod <name of file without .rs>` at the top of the file. 

#### Lesson 35 - Regular Expressions
Crate name : l35-regex

We are using regular expressions to see if there is a match. For example,
the expression we are using is the `\w{5}` to find if the word entered matches
the 5-letter rule. 

We have also used the capture method to capture what the word is that passes
the regex rule. 

#### Lesson 36 - Modules
Crate name : l36-modules

Modules allow you to separate code into neat sections. We define modules by 
using the `mod` keyword. By default, all functions inside a module are private
so we need to add a `pub` in front of the function.  

We can have a public function call a private function and we can have modules 
inside of a module. This can be called as `mod1::mod2::mod2_func`.

#### Lesson 37 - Option (Enums)
Crate name : l37-option

Option represents either a value or no value at all. The two keywords to
understand are Some and None. Some represents an option while None represents
no option. We can have multiple Some but only one None. 

We can even return an option depending on the situation. 

#### Lesson 39 - Enum Methods
Crate name : l39-enum-methods

We can move logic from main functions into enum types. First we need to 
create an enum type. Then we create an implement function type `impl` with a 
function. This function is going to check the enum passed in and return a bool
value.  

Note: When creating an OR expression, use one | not two ||. 

#### Lesson 40 - CLI (Command Line Interface)
Crate name : l40-cli

We are going to be running or executing commands using the command line
interface within the rust file. To do this, I have created a sample python
script called hello.py which all it does is print hello world.

We need to use the `std::process::Command` struct.  

Note: I have created a Command block that calls the python3 command and runs it
on hello.py. The issue with the actual tutorial from YouTube is the use of the
unsafe function and the relative path of the python3 command. To fix this, I
am giving the absolute path of python3 which is /usr/bin/python3 to fix any 
alias issues. As for the unsafe command, I am using from_utf8_lossy function 
to convert the output from bytes to utf8-string. 

The function String::from_utf8_lossy converts a slice of bytes to a string including
invalid characters. This will replace any invalid UTF-8 sequences with a ? character.
Therefore, the return output needs to be a valid UTF-8 string. 

#### Lesson 41 - Writing and Running Tests
Crate name : l41-tests

We can run our tests using `cargo test`. To write a test, we can define a 
separate module for tests. Each test needs to have the header `#[test]`. There
are other headers we can use too to do certain functions but please see the code
for those headers.  

NOTE: When calling out of scope functions, it is important to use the super
keyword. So when testing functions, write super::<function>. 

This super keyword also works for structs too. This allows us to write tests
for structs and their implementation function. 
