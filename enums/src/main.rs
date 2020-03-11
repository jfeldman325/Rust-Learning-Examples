enum IpaddrKind {
    V4,
    V6,
}

struct IpAddr {
    version: IpaddrKind,
    address: String,
    mask: String,
}

impl IpAddr {
    fn new(version: IpaddrKind, address: String, mask: Option<String>) -> IpAddr {
        match mask {
            Some(x) => {
                return IpAddr {
                    version,
                    address,
                    mask: x,
                }
            }
            None => {
                return IpAddr {
                    version,
                    address,
                    mask: "255.255.255.255".to_string(),
                }
            }
        }
    }
    fn new_v4(address: String, mask: Option<String>) -> IpAddr {
        IpAddr::new(IpaddrKind::V4, address, mask)
    }
    fn new_v6(address: String, mask: Option<String>) -> IpAddr {
        IpAddr::new(IpaddrKind::V6, address, mask)
    }
}

fn main() {
    let four = IpaddrKind::V4;
    let six = IpaddrKind::V6;

    let loopback = IpAddr::new_v4("127.0.0.1".to_string(), None);
    println!("{}", loopback.mask);
}
