fn main() {
    let number = 7;// if input is integer then conditions should also integer and same for other datatypes

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true; // passing if to let
    let number = if condition {
        "five" // or 5
    } else {
        "six"  // or 6 but be same datatype
    };

    println!("The value of number is: {}", number);
}
