// Using Tuple Structs Without Named Fields to Create Different Types

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/*
    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
*/

// Unit-Like Structs Without Any Fields

/*
    struct AlwaysEqual;
    
    fn main() {
        let subject = AlwaysEqual;
    }
*/


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("SomeUserName123"),
        email: String::from("someEmail@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("updateEmail@example.com");

    println!("The email of {} is: {}", user1.username, user1.email);

    let user2 = User {
        username: String::from("AnotherUserName123"),
        email: String::from("another@example.com"),
        ..user1
    };

    println!("The email of {} is: {}", user2.username, user2.email);

    let user3 = build_user(String::from("buildedUser@example.com"), String::from("BiuldedUser123"));
    println!("The email of {} is: {}", user3.username, user3.email);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
