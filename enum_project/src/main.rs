use std::net::IpAddr;

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    // route(four);
    // route(six);

    let home = IpAddrs {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let home2 = IpAddrs {
        kind: six,
        address: String::from("::1"),
    };
    // Option
    // 如果不声明 泛型 Option会自己识别
    let some_number = Some(5);
    let some_string = Some("A String");
    // 类似其他语言的null
    let absent_number: Option<i32> = None;

    // Option 作用是 被Option修饰的变量  类型不相等是不能相加减
}

enum Coin {
    Penny,
    Nickel,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
    }
}
// rust 允许数据附加到枚举的变体中
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
// enum 也可以使用impl定义方法
impl IpAddrKind {
    fn call(&self) {}
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddrs {
    kind: IpAddrKind,
    address: String,
}
