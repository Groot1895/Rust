fn main()
{
	let mut x =5.2+6.9; // integer and float cannot be mixed
	println!("value of x is {}",x);
	x=6.0;
	println!("value of x is {}",x);
	let y = {
        let x = 3;
        x + 1 //return statement has no (;)
    };

    println!("The value of y is: {}", y);
}
