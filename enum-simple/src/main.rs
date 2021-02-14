enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("IP addr v4:  {} ", match_home(home));
    println!("IP addr v6:  {} ", match_home(loopback));
}

fn match_home(ipaddr: IpAddrKind) -> String {
    match ipaddr {
        IpAddrKind::V4(addr) => {
           addr
        }
        IpAddrKind::V6(addr) => {
            addr
        }
    }
}
