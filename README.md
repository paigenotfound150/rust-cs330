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
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Using with Visual Studio Code
[5] After installing Rust on your computer by using the instructions above, follow the next steps to use with Visual Studio Code. 
1. Install Visual Studio Code on your machine here: https://code.visualstudio.com/download

2. Install the rust analyzer extension: within the Extensions tab on VS Code, search for 'rust-analyzer' and install the latest version.
3. Run 'rustc --version'. You should see something like `rustc 1.72.0 (5680fa18f 2023-08-23)` in the console.
4. When you want to update, you can run 'rustup update'
5. Cargo is a Rust Package manager that comes for free when we first installed Rust using Rustup. We'll be using this to compile and run our code, and to create our first "Hello World" program later on.

#### Hello World
1. We'll be using Cargo, as mentioned earlier!
2. Using the terminal, navigate to the directory you'd like to create your program in. We're going to create a new cargo project by running the following command:
   `cargo new hello_world`
4. Next we will navigate to our new project and open it in Visual Studio Code.
   `cd hello_world`
   `code .`
5. Navigate into the src folder and press on main.rs. This is the file we'll be running our program from.
6. As you can see, there is our main function that prints out to the console using !println
   `fn main() {
       println!("Hello, World");
    }`
 7.  In the VS Code integrated terminal, run `cargo build`. This will deploy our build output into an executable called "hello_world.exe".
 8.  To run our project and print out our "Hello World" to the console, run the following command:
    `cargo run`. 

## Sources
[1] https://en.wikipedia.org/wiki/Rust_(programming_language)#:~:text=Rust%20is%20a%20multi%2Dparadigm,in%20other%20memory%2Dsafe%20languages.

[2] https://en.wikipedia.org/wiki/Servo_(software)#:~:text=Servo%20was%20the%20engine%20of,browser%20was%20released%20in%202020.

[3] https://forge.rust-lang.org/infra/other-installation-methods.html

[4] https://www.rust-lang.org/learn/get-started

[5] https://code.visualstudio.com/docs/languages/rust

