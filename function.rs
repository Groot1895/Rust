fn main() {
    display(45,1);
    let x = plus_one(5,1);
    println!("function with return value {}", x);
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); //return from function
    let s = String::from("hell0o");
    let le = calculate_len(&s);
    println!("The length of s '{}' is {}.", s, le);
    println!("The length of s1 '{}' is {}.", s2, len);
}

fn display(x:i8,y:i32)
{
    println!("inside display function with passing parameters {} {}",x,y);
}

fn plus_one(x: i8,y: i32) -> i32 {
    x as i32 + y + 1 //return does't have (;)

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
