enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    // We automatically get this constructor function
    // defined as a result of defining the enum.
}

enum Message {
    Quit,
    Move { x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("Call method.")
    }
}

fn main() {
    let m = Message::Quit;

    m.call();

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // Option values 
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

}

fn route(ip_kind: IpAddrKind){

}