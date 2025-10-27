enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddres {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
    let some_num = Some(4);
}

fn route(ip_kind: IpAddrKind) {}
