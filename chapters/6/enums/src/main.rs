enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// A helpful Rust enum is the Option<T> enum. It replaces the null value in other languages.
// Rust avoids null as it leads to many runtime errors where developers don't consider null
// values and raise errors when accessing them. Instead, Option<T> is used to represent when
// a value can be present or absent but because of this deterministic behavior, the developer
// has to handle the case when the value is absent.
// Its definition is as follows, it's actually already preluded in the standard library:
// enum Option<T> {
//     Some(T),
//     None,
// }

impl Message {
    fn print(&self) {
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Move to ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Write: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("New Color: R{} G{} B{}", r, g, b);
            }
        }
    }
}

fn main() {
    let local4 = IpAddr::V4(127, 0, 0, 1);
    let local6 = IpAddr::V6(String::from("::1"));

    print_ip(&local4);
    print_ip(&local6);

    let quit = Message::Quit;
    let move_to = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Hello, World!"));
    let change_color = Message::ChangeColor(180, 200, 80);

    quit.print();
    move_to.print();
    write.print();
    change_color.print();

    let some_string = Some(String::from("A present string!"));
    let absent_string: Option<String> = None;

    print_some_string(&some_string);
    print_some_string(&absent_string);
    alt_print_some_string(&absent_string);
}

fn print_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(i1, i2, i3, i4) => println!("{}.{}.{}.{}", i1, i2, i3, i4),
        IpAddr::V6(ip6) => println!("{}", ip6),
    }
}

fn print_some_string(s: &Option<String>) {
    match s {
        Some(s) => println!("{}", s),
        None => println!("None"),
    }
}

fn alt_print_some_string(s: &Option<String>) {
    println!("{}", s.as_ref().unwrap_or(&String::from("None")));
}
