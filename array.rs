fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    //let a: [i32; 5] = [1, 2, 3, 4, 5]; // type of each element
    let a1 = [3; 5]; // all 5 elements are 3
    let first = a[0];// way of accessing ARRAY
    println!("{:?}\n{:?}\n{:?}\n{:?}\n",a,months,a1,first);
}
