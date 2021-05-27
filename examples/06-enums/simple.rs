use std::io::*;

fn route_ip(ip: IPAddr) {
    match ip {
        IPAddr::V4(first, second, third, fourth) => println!("Sending v4 packet: {}.{}.{}.{}", first, second, third, fourth),
        IPAddr::V6(addr) => println!("Sending v6 packet: {}", addr),
    }
}

enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
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

fn main() {
    // let home = IPAddr::V4(127, 0, 0, 1);
    let mut addr_str = String::new();
    stdin().read_line(&mut addr_str);
    let home = IPAddr::new_v4(addr_str.trim());

    let loopback = IPAddr::V6(String::from("::1"));

    route_ip(home);
    route_ip(loopback);

    // let loopback = IPAddr{
    //     kind: IPAddrKind::V6,
    //     address: "::1",
    // }
}
