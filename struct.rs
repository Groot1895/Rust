// usual structure
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//shorthand to apply constant values in structure
fn build_user(email: String, username: String) -> User {
    User {
            email,
            username,
            active: true,
            sign_in_count: 1, }
}

//tuple structure
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

//method syntax --> structure Rectangle alone can access this
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 2,
    };

    let user2 = build_user (
        String::from("another@example.com"),
        String::from("anotherusername567"),
    );

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // this to copy missing field from other instance
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangles is {} {} square pixels.",
        rect1.area(), rect2.area()
    );

	println!("{} {} {} {} {} {}",user1.email,user2.username,user3.sign_in_count,user2.active,black.0,origin.0);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
