fn main() {
    let x: bool = true;
    let y: bool = false;

    if x == true {
        println!("Hello");
    } else if y == false {
        println!(" World");
    }

    let num = 1
    match num{
        1=>println!("One"),
        2=>println!("Two"),
        3=>println!("Three"),
        4=>println!("Four"),
        _=>println!("The rest"),
    }

    match num{
        1|2->println!("One or two"),
        3|4=>println!("Three or four"),
        _=>println!("The rest"),
    }

    match num{
        1..=4=>println!("1 through 5");
        _=>println!("The rest")
    }
}
