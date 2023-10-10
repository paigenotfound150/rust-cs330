fn main() {
    let product = multiply(5, 3);
    let recursive = recursive_factorial(4);
    let split_sring = split_string("hello world".to_owned());
    let test = 500;
    test_pass_by(test);
    println!("{}", test);
}

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

