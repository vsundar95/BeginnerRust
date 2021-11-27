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
