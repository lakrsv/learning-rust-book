fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let home = IpAddrEnum::V4_INT(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Move { x: 5, y: 2 };
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // let sum = some_number + 5; // Nope

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Hello 3"),
        7 => println!("Hello 7"),
        _ => println!("Other"),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // Instead
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddrEnum {
    V4(String),
    V4_INT(u8, u8, u8, u8),
    V6(String),
}

struct IpV4Addr {}
struct IpV6Addr {}
enum IpAddrEnumStruct {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
