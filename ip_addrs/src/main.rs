fn main() {
    enum IpAddrVersion {
        V4,
        V6,
    }

    struct IpAddr {
        version: IpAddrVersion,
        address: String,
    };

    let home = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1"),
    };
}
