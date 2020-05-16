fn main() {
    /*loop {
        println!("again!");
    }*/
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
//---------------------- WHILE --------------------------------------------------------------------
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
//-------------------- FOR ------------------------------------------------------------------------
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..8).rev() { //reverse loop
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
