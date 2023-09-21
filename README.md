# Rust-CS330
This repo is for learning Rust in Programming Languages - CS330 at Simmons University.
Author: Paige Thompson


![Rust Programming Icon](https://www.rust-lang.org/static/images/rust-social-wide.jpg)

## Language overview and setup
Rust is a high-level, general-purpose programming language that enforces statically typed variables and memory safety. 
### History
[1] Rust was named after an incredibly hardy fungus, the Rust fungus, after being created by Mozilla employee Graydon Hoare in 2006. It began as a personal project and became sponsored by Mozilla in 2009 as part of Mozilla's 2010 browser engine, Servo. [2] Servo became the browser engine for two Mozilla projects, the augmented reality browser for the Magic Leap headset in 2018 and then the FireFox reality headset released two years later.

By October 2012, Rust had classes, destructors, polymorphism, inheritance, and structured types. Memory management was also improved shortly after. The first stable release, Rust 1.0, was released in 2015. Rust is mainly used for developing complex applications such as gaming engines, operating systems, and browsers, however it is also used for web development and embedded systems.
I will be using the below sources for my learning:
 1. Learn Rust Programming - Progamiz
    - https://www.programiz.com/rust
 3. The Rust Programming Language - by Steve Klabnik and Carol Nichols
    - https://doc.rust-lang.org/book/

### Getting started

#### Installation on Windows
[3] Follow the instructions here to download and install rustup-init.exe: https://forge.rust-lang.org/infra/other-installation-methods.html

#### Installation on MacOS
[4] You can get started by using the Rust installer and version management tool "Rustup", by running the following command: 
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

#### Using with Visual Studio Code
[5] After installing Rust on your computer by using the instructions above, follow the next steps to use with Visual Studio Code. 
1. Install Visual Studio Code on your machine here: https://code.visualstudio.com/download

2. Install the rust analyzer extension: within the Extensions tab on VS Code, search for 'rust-analyzer' and install the latest version.
3. Run 'rustc --version'. You should see something like `rustc 1.72.0 (5680fa18f 2023-08-23)` in the console.
4. When you want to update, you can run 'rustup update'
5. Cargo is a Rust Package manager that comes for free when we first installed Rust using Rustup. We'll be using this to compile and run our code, and to create our first "Hello World" program later on.

#### Hello World
We'll be using Cargo, as mentioned earlier!
1. Using the terminal, navigate to the directory you'd like to create your program in. We're going to create a new cargo project by running the following command:
   ```cargo new hello_world```
2. Next we will navigate to our new project and open it in Visual Studio Code.
   ```cd hello_world```
   
   ```code .```
4. Navigate into the src folder and press on main.rs. This is the file we'll be running our program from.
5. As you can see, there is our main function that prints out to the console using !println
   ```fn main() { println!("Hello, World"); }```
 6.  In the VS Code integrated terminal, run `cargo build`. This will deploy our build output into an executable called "hello_world.exe".
 7.  To run our project and print out our "Hello World" to the console, run the following command:
   ```cargo run```

#### Writing Comments
To write comments in a Rust file, use two forward slashes //

### Data Types and Naming Conventions

#### Naming Conventions
Modules: snake_case
Types: CamelCase
Enums: CamelCase
Functions: snake_case
Local Variables: snake_case
Constant Variables: SCREAMING_SNAKE_CASE

#### Mutability 
Variables in Rust are immutable by default, so once you declare a value to a variable name, you can't change it.
`let x = 5; // Immutable
 let mut x = 5; // Mutable
 `
Rust allows you to 'shadow', or cast the value of one variable on top of another variable. 
` let x: i32 = 5;

  x: i32 = 5 + 1; // Will get error: 'cannot assign twice to immutable variable'
  
  let x: i32 = x + 1; // Will work
 `
 
#### Data Types

##### Integers

 Length	  Signed	   Unsigned
8-bit	     i8	        u8
16-bit	    i16	       u16
32-bit	    i32	       u32
64-bit	    i64	       u64
128-bit	   i128	      u128
arch	      isize	     usize
 
Integer is any number that is not a fraction. The above table shows the different Integer types in Rust.
'Signed' means that the number may become positive or negative, and 'unsigned' means it will only ever be positive.
'Signed' numbers can store numbers -(2^(n - 1)) to (2^(n - 1) - 1), where n is the number of bits that 



string
floating-point number
boolean
array/list
dictionary (sometimes called a hash or a map, depending on your language)


 In your example code, illustrate the important data type and operations features in your language. Write code that experiments with different operations applied on variables of the same data type and operations with variables of two different types: e.g. can you add ints and floats? Is the resulting variable an int (narrowing conversion) or a float (widening conversion)?  What about division? Can you put different data types in the same array or list?  Can one data type be converted to another either implicitly or explicitly (int to float, string to int, etc)? 

 

Discussion questions:

Does your language have keywords or reserved words? How many?
What are the naming requirements for variables in your language? 
What about naming conventions?  Are those enforced by the compiler/interpreter, or just standards in the community?

Is your language statically or dynamically typed?
Strongly typed or weakly typed?
Are some variables mutable while others are immutable? 
What are the operators available for each data type?
Are mixed type operations allowed? If so, how are they accommodated?
At what point are identifier names and operator symbols bound in your language? For example if you declare a (variable, class name, function name), when is it bound to the type, address? When are operators (+,*, etc.) bound to their operations?
Explicitly typed or implicitly typed? 
 

CODING EXAMPLE demonstrating the features above

 

EXAMPLE OF (one) ILLUSTRATIVE EXAMPLE:

If you put this line (or something similar) in a program and try to print x, what does it do? 

x = "5" + 6

If it doesn't compile, why?  Is there something you can do to make it compile?

 

Describe the limitations (or lack thereof) of your programming language as they relate to the coding example portion of the assignment (adding ints and floats, storing different types in lists, converting between data types).  Are there other restrictions or pitfalls that the documentation mentions that you need to be aware of?
Are there built-in complex data types that are commonly used in your language? (hint: they’d probably appear fairly early in the documentation if so)
 

## Sources
[1] https://en.wikipedia.org/wiki/Rust_(programming_language)#:~:text=Rust%20is%20a%20multi%2Dparadigm,in%20other%20memory%2Dsafe%20languages.

[2] https://en.wikipedia.org/wiki/Servo_(software)#:~:text=Servo%20was%20the%20engine%20of,browser%20was%20released%20in%202020.

[3] https://forge.rust-lang.org/infra/other-installation-methods.html

[4] https://www.rust-lang.org/learn/get-started

[5] https://code.visualstudio.com/docs/languages/rust

[6] https://doc.rust-lang.org/book/ch03-02-data-types.html

