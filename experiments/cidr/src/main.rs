use std::{
    fmt,
    io::{self, Write as _},
    net::Ipv4Addr,
    str::FromStr,
};

fn main() {
    let cidr: CidrV4 = input("Enter the CIDR address: ");

    println!("For CIDR:        {cidr}:");
    println!("Network size:    {}", cidr.network_size());
    println!("Network ID:      {}", cidr.network_id());
    println!("Broadcast:       {}", cidr.broadcast());
    println!(
        "Next network ID: {}",
        cidr.next_network_id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!("First host:      {}", cidr.first_host());
    println!("Last host:       {}", cidr.last_host());
}

fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

struct CidrV4 {
    addr: Ipv4Addr,
    prefix: u8,
}

impl CidrV4 {
    fn addr_bits(&self) -> u32 {
        let octets: [u8; 4] = self.addr.octets();
        u32::from_be_bytes(octets)
    }

    fn host_bit_count(&self) -> u32 {
        32 - (self.prefix as u32)
    }

    fn network_range(&self) -> u32 {
        self.network_size() - 1
    }

    pub fn network_size(&self) -> u32 {
        2_u32.pow(self.host_bit_count())
    }

    fn mask_bits(&self) -> u32 {
        u32::MAX << self.host_bit_count()
    }

    fn id_bits(&self) -> u32 {
        self.addr_bits() & self.mask_bits()
    }

    pub fn network_id(&self) -> Ipv4Addr {
        self.id_bits().into()
    }

    pub fn broadcast(&self) -> Ipv4Addr {
        (self.id_bits() + self.network_range()).into()
    }

    pub fn next_network_id(&self) -> Option<Ipv4Addr> {
        self.id_bits()
            .checked_add(self.network_size())
            .map(Into::into)
    }

    pub fn first_host(&self) -> Ipv4Addr {
        (self.id_bits() + 1).into()
    }

    pub fn last_host(&self) -> Ipv4Addr {
        (self.id_bits() + self.network_range() - 1).into()
    }
}

impl FromStr for CidrV4 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((addr, prefix)) = s.split_once('/') else {
            return Err("missing slash");
        };
        let addr = addr.parse().map_err(|_| "invalid addr")?;
        let prefix = prefix
            .parse()
            .ok()
            .filter(|n| (0..=32).contains(n))
            .ok_or("invalid prefix")?;
        Ok(CidrV4 { addr, prefix })
    }
}

impl fmt::Display for CidrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        cidr: &'static str,
        network_size: u32,
        network_id: Ipv4Addr,
        broadcast: Ipv4Addr,
        next_network_id: Option<Ipv4Addr>,
        first_host: Ipv4Addr,
        last_host: Ipv4Addr,
    }

    #[test]
    fn test() {
        let cases = &[
            Case {
                cidr: "10.1.1.37/29",
                network_size: 8,
                network_id: Ipv4Addr::from([10, 1, 1, 32]),
                broadcast: Ipv4Addr::from([10, 1, 1, 39]),
                next_network_id: Some(Ipv4Addr::from([10, 1, 1, 40])),
                first_host: Ipv4Addr::from([10, 1, 1, 33]),
                last_host: Ipv4Addr::from([10, 1, 1, 38]),
            },
            Case {
                cidr: "10.2.2.88/27",
                network_size: 32,
                network_id: Ipv4Addr::from([10, 2, 2, 64]),
                broadcast: Ipv4Addr::from([10, 2, 2, 95]),
                next_network_id: Some(Ipv4Addr::from([10, 2, 2, 96])),
                first_host: Ipv4Addr::from([10, 2, 2, 65]),
                last_host: Ipv4Addr::from([10, 2, 2, 94]),
            },
            Case {
                cidr: "213.50.111.222/2",
                network_size: 1_073_741_824,
                network_id: Ipv4Addr::from([192, 0, 0, 0]),
                broadcast: Ipv4Addr::from([255, 255, 255, 255]),
                next_network_id: None,
                first_host: Ipv4Addr::from([192, 0, 0, 1]),
                last_host: Ipv4Addr::from([255, 255, 255, 254]),
            },
        ];

        for case in cases {
            let cidr = CidrV4::from_str(&case.cidr).expect("should parse cidr");
            assert_eq!(cidr.network_size(), case.network_size);
            assert_eq!(cidr.network_id(), case.network_id);
            assert_eq!(cidr.broadcast(), case.broadcast);
            assert_eq!(cidr.next_network_id(), case.next_network_id);
            assert_eq!(cidr.first_host(), case.first_host);
            assert_eq!(cidr.last_host(), case.last_host);
        }
    }
}
