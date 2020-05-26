fn main() {
    //let s = String::new();  ---> creating null string <---
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    let mut s = String::from("lo");
    s.push('l');
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4;
    let s6 = format!("{}-{}-{}", s1, s2, s5); // format macro used to combine strings with special characters
    println!(" {} {} {} {} {} {}", s2, s1, s, s4, s5, s6);
    for c in "வணக்கம்".chars() {
    println!("{}", c);
    }
    for c in "வணக்கம்".bytes() {
    println!("{}", c);
    }
}
