// Rfactor so that the enum variants will have associated
// String values
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Similar to defining different kinds of struct definitions
// except the enum doesn't use the struct keyword and all
// the variants are grouped togheter under the Message type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// fn route(ip_kind: IpAddrKind) {}

fn main() {
    // The following example will work with the
    // original non-refactored code
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
