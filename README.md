# Rust-CS330
This repo is for learning Rust in Programming Languages - CS330 at Simmons University.
Author: Paige Thompson

<a href="url"><img src="[http://url.to/image.png](https://www.rust-lang.org/static/images/rust-social-wide.jpg)" align="left" height="48" width="48" ></a>

## Language overview and setup
Rust is a high-level, general-purpose programming language that enforces statically typed variables and memory safety. 
### History
[1] Rust was named after an incredibly hardy fungus, the Rust fungus, after being created by Mozilla employee Graydon Hoare in 2006. It began as a personal project and became sponsored by Mozilla in 2009 as part of Mozilla's 2010 browser engine, Servo. [2] Servo became the browser engine for two Mozilla projects, the augmented reality browser for the Magic Leap headset in 2018 and then the FireFox reality headset released two years later.

By October 2012, Rust had classes, destructors, polymorphism, inheritance, and structured types. Memory management was also improved shortly after. The first stable release, Rust 1.0, was released in 2015. 

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

## Sources
[1] https://en.wikipedia.org/wiki/Rust_(programming_language)#:~:text=Rust%20is%20a%20multi%2Dparadigm,in%20other%20memory%2Dsafe%20languages.

[2] https://en.wikipedia.org/wiki/Servo_(software)#:~:text=Servo%20was%20the%20engine%20of,browser%20was%20released%20in%202020.

[3] https://forge.rust-lang.org/infra/other-installation-methods.html

[4] https://www.rust-lang.org/learn/get-started

[5] https://code.visualstudio.com/docs/languages/rust

