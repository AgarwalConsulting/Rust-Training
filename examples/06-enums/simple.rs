use std::io::*;

// #[derive(Debug)]
// enum IPAddrKind {
//     V4,
//     V6
// }

// #[derive(Debug)]
// struct IPAddr {
//     addr: String,
//     kind: IPAddrKind
// }

// fn route_ip(ip: IPAddr) {
//     let kind = ip.kind;

//     match kind {
//         IPAddrKind::V4 => println!("V4 routing to IP: {}", ip.addr),
//         IPAddrKind::V6 => println!("V6 routing to IP: {}", ip.addr)
//     }
// }

#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn route_ip(ip: IPAddr) {
    match ip {
        IPAddr::V4(first, second, third, fourth) => println!("Sending v4 packet: {}.{}.{}.{}", first, second, third, fourth),
        IPAddr::V6(addr) => println!("Sending v6 packet: {}", addr),
    }
}

impl IPAddr {
    fn new_v4(s: &str) -> IPAddr {
        let mut byte = String::new();
        let mut t: [u8; 4] = [0; 4];
        let mut index = 0;

        for digit in s.chars() {
            if digit == '.' {
                t[index] = byte.parse().unwrap();
                byte = String::new();
                index+= 1;
            } else {
                byte.push(digit);
            }
        }

        t[index] = byte.parse().unwrap();

        // let bytes: Vec<u8> = s.split(".").map(String::into).collect();

        IPAddr::V4(t[0], t[1], t[2], t[3])
    }
}

impl IPAddr {
    fn route(&self) {
        match self {
            IPAddr::V4(127, 0, 0, 1) => println!("V4 routing locally..."),
            IPAddr::V4(255, 255, 255, 255) => println!("V4 broadcast"),
            IPAddr::V4(b1, b2, b3, b4) => println!("V4 routing to IP: {}.{}.{}.{}", b1, b2, b3, b4),
            IPAddr::V6(addr) => match addr.as_str() {
                "::1" => println!("V6 routing locally..."),
                _ => println!("V6 routing to IP: {}", addr)
            }
        }
    }
}

fn main() {
    // let loopback_v4 = IPAddr{addr: String::from("127.0.0.1"), kind: IPAddrKind::V4};
    let loopback_v4 = IPAddr::V4(127, 0, 0, 1);

    // let loopback_v6 = IPAddr{addr: String::from("::1"), kind: IPAddrKind::V6};
    let loopback_v6 = IPAddr::V6(String::from("::1"));

    println!("{:?}", loopback_v4);
    // route_ip(loopback_v4);
    // IPAddr::route(loopback_v4);

    loopback_v4.route();

    println!("{:?}", loopback_v6);
    // route_ip(loopback_v6);
    // IPAddr::route(loopback_v6);

    loopback_v6.route();

    IPAddr::V4(255, 255, 255, 255).route();
    IPAddr::V4(192, 168, 1, 116).route();

    IPAddr::V6(String::from("::2")).route();
}
