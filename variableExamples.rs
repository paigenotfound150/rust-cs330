fn main() {
    //integer
    let x = 5;
    // float
    let y = 5.0;
    // boolean
    let trigger = false;
    // char
    let character = 'x';
    // string
    let string = "Hello"; 
    // array
    let array = [1,2,3,4];
    // tuple
    let tuple = ("Hey", 40, 'h', 39.9);

    // all variables are strongly typed

    // casting an int as a float
    let z = x as f64; //f64 is a float type
    // casting a float as an int
    let c = y as u8;

    // String to Integer
    let new_int: u8 = string.parse().unwrap(); // Need to explicitly define int type
    // Integer to String
    x.to_string();
}
