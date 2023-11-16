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
3. Next we will navigate to our new project and open it in Visual Studio Code.

   ```cd hello_world```
   
   ```code .```
5. Navigate into the src folder and press on main.rs. This is the file we'll be running our program from.
6. As you can see, there is our main function that prints out to the console using !println

   ```
   
   fn main() {
       println!("Hello, World");
      }
   ```
   
 9.  In the VS Code integrated terminal, run `cargo build`. This will deploy our build output into an executable called "hello_world.exe".
 10.  To run our project and print out our "Hello World" to the console, run the following command: ```cargo run```

#### Writing Comments
To write comments in a Rust file, use two forward slashes //




## Data Types and Naming Conventions

### Naming Conventions
Rust is case-sensitive, and variables must begin with either a letter or an underscore.
Modules: snake_case
Types: CamelCase
Enums: CamelCase
Functions: snake_case
Local Variables: snake_case
Constant Variables: SCREAMING_SNAKE_CASE

Rust has 37 reserved words. 
[List of all reserved keywords in Rust](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/appendix-01-keywords.html)

### Mutability 
Variables in Rust are immutable by default, so once you declare a value to a variable name, you can't change it.

```let x = 5; // Immutable```

 ```let mut x = 5; // Mutable```
 
Rust allows you to 'shadow', or cast the value of one variable on top of another variable. 

``` let x: i32 = 5;```
  
```let x: i32 = x + 1;  // x is now equal to 6```

#### Rust is statically and strongly typed.
 
### Data Types
##### Scalar Types
- Integer
```let x = 10;```
- Floating Point
```let y = 5.0;```
- Boolean
```let toggle = true;```
- Character
```let char = 'x';```
```let specialChar = '@'```
- String
```let string = "Hello"```

##### Compound Types
- Tuple:
  Allows us to store values of different data types together.
  
  ```let tuple = ('String', 15.0, 15);```
  
  Tuples have fixed sizes and elements cannot be added or removed after creation.
  
- Array:
  Allows us to store values of the same data type.
  
  ```let array = [4,10,5,3,1]```

  Arrays are immutable by default, but we can create mutable arrays with the mut keyword.

#### Operations
Cannot perform arithmetic between float and integer
  
```
let x = 5;
let y = 10;
x + y; // error: cannot add a float to an integer
```

#### Type Conversions
 - You can't convert integers to floats (and vice versa), but you can cast the desired type onto a new variable.
Converting integer to float:
```
let x = 5;
let y = x as f64; //f64 is a 64-bit floating point type
println!("{}", y + 5.0); // 10.0
```

Converting from string to integer

String -> Integer: "string".parse().unwrap(); 

Integer -> String: integer_variable.to_string();

- Mixed type operations are not allowed.



## Functions [7]
Rust uses snake_case for naming functions.
A function is defined by using fn, then the function name, then the parameter names (preceded by type).

```
//Functiont to multiply two numbers
fn multiply(x:i32, y:i32) -> i32 {
    return x * y;
}

//Function to get the factorial of a number using recursion
fn recursive_factorial(x:i32) -> i32 {
    if x == 1 {
        return x
    }
    return x * recursive_factorial(x-1);
}

//Function to split a string based on whitespace, returns a Vector of strings
fn split_string(string: String) -> Vec<String> {
    let new_string: Vec<String> = string.split(" ").map(|s| s.to_string()).collect();
    return new_string;
}

//Function to test pass by value or pass by reference
fn test_pass_by(mut x: i32) {
    x = 900;
}
```

- Rust doesn't care where in your file you define your functions, only that they are within scope.
- Rust supports recursive functions.
- Functions can accept multiple parameters with different data types.
- Functions can return multiple values through a Vector, Tuple, or an Array.
- Functions in Rust are pass by value, not pass by reference.
- Arguments, parameters, and local variables are stored by value on the stack during execution.
- Variables are not accessible after or before code block execution, only during.
- Variables inside of functions are immutable by default - you have to use the mut keyword during declaration to make them mutable.

## Selection, Loops, Conditionals
### Booleans
```
    let x: bool = true;
    let y: bool = false;
```
### Conditionals 
```

    if x == true {
        // add logic here
    } else if y == false {
        // add more logic
    } else {
        // add more logic
    }

    if x == true && y == true {
        // do something
    }
```

Rust supports short circuit operations. The below snippet of code will print out "Hello", since the conditional stops executing once it finds the true evaluation.
```
    let x: bool = true;
    let y: bool = false;

    if x == true {
        println!("Hello");
    } else if y == false {
        println!(" World");
    }
```

### Switch Statements
Rust has several ways to write switch statements.

- Single value matching
```
    let num = 1
    match num{
        1=>println!("One"),
        2=>println!("Two"),
        3=>println!("Three"),
        4=>println!("Four")
    }
```

- Multiple value matching
```
    match num{
        1|2->println!("One or two"),
        3|4=>println!("Three or four"),
        _=>println!("The rest"),
    }
```

- Matching a value that exists in a range (both ends are included)
```
    match num{
        1..=4=>println!("1 through 5");
        _=>println!("The rest")
    }
```


## Classes 
- Rust uses Structs and Enums instead of Classes for creating custom types.
- Rust uses UpperCamelCase for its structs.
- Rust uses snake_case for function and variable names.
  
Creating a struct:
```
   struct Dog {
     name: String,
     age: u32,
   }
```
Creating an instance of a struct:
```
   let dog = Dog {
       name: "Jerry",
       age: 12,
  }
```

- Instead of using methods for classes, Rust uses "impl blocks" for its structs and enums.

```
   impl Dog {
     // Init method
     fn new(name: &str, age: u32) -> Dog {
        Dog {
            title: String::from(name),
            age,
        }
    }

    // An instance method
    fn printName(&self) {
        println!("'{}', self.name);
    }
 }
```
The "new" method is a standard method for creating your object.

Rust does not use inheritance like OOP languages, but you can give a struct instances of another struct.
For example, a Person struct may have a Dog. 
```
struct Person {
  dog: Dog,
}
```



## Sources
[1] https://en.wikipedia.org/wiki/Rust_(programming_language)#:~:text=Rust%20is%20a%20multi%2Dparadigm,in%20other%20memory%2Dsafe%20languages.

[2] https://en.wikipedia.org/wiki/Servo_(software)#:~:text=Servo%20was%20the%20engine%20of,browser%20was%20released%20in%202020.

[3] https://forge.rust-lang.org/infra/other-installation-methods.html

[4] https://www.rust-lang.org/learn/get-started

[5] https://code.visualstudio.com/docs/languages/rust

[6] https://doc.rust-lang.org/book/ch03-02-data-types.html

[7] https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

[8] https://www.geeksforgeeks.org/rust-switch-case/#



