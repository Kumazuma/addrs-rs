#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod af;
mod c_function;
mod ifa;

mod c_type;
use c_function::*;
use c_type::*;
use ifa::*;
#[allow(non_snake_case)]
pub mod AF {
    pub use super::af::AddressFamily;
    /* Protocol families.  */
    pub const UNSPEC: AddressFamily = AddressFamily::UNSPEC; /* Unspecified.  */
    pub const LOCAL: AddressFamily = AddressFamily::LOCAL; /* Local to host (pipes and file-domain).  */
    pub const UNIX: AddressFamily = AddressFamily::LOCAL; /* POSIX name for PF_LOCAL.  */
    pub const FILE: AddressFamily = AddressFamily::LOCAL; /* Another non-standard name for PF_LOCAL.  */
    pub const INET: AddressFamily = AddressFamily::INET; /* IP protocol family.  */
    pub const AX25: AddressFamily = AddressFamily::INET; /* Amateur Radio AX.25.  */
    pub const IPX: AddressFamily = AddressFamily::INET; /* Novell Internet Protocol.  */
    pub const APPLETALK: AddressFamily = AddressFamily::INET; /* Appletalk DDP.  */
    pub const NETROM: AddressFamily = AddressFamily::NETROM;
    pub const BRIDGE: AddressFamily = AddressFamily::BRIDGE;
    pub const ATMPVC: AddressFamily = AddressFamily::ATMPVC;
    pub const X25: AddressFamily = AddressFamily::X25;
    pub const INET6: AddressFamily = AddressFamily::INET6;
    pub const ROSE: AddressFamily = AddressFamily::ROSE;
    #[allow(non_upper_case_globals)]
    pub const DECnet: AddressFamily = AddressFamily::DECnet;
    pub const NETBEUI: AddressFamily = AddressFamily::NETBEUI;
    pub const SECURITY: AddressFamily = AddressFamily::SECURITY;
    pub const KEY: AddressFamily = AddressFamily::KEY;
    pub const NETLINK: AddressFamily = AddressFamily::NETLINK;
    pub const ROUTE: AddressFamily = AddressFamily::NETLINK;
    pub const PACKET: AddressFamily = AddressFamily::PACKET;
    pub const ASH: AddressFamily = AddressFamily::ASH;
    pub const ECONET: AddressFamily = AddressFamily::ECONET;
    pub const ATMSVC: AddressFamily = AddressFamily::ATMSVC;
    pub const RDS: AddressFamily = AddressFamily::RDS;
    pub const SNA: AddressFamily = AddressFamily::SNA;
    pub const IRDA: AddressFamily = AddressFamily::IRDA;
    pub const PPPOX: AddressFamily = AddressFamily::PPPOX;
    pub const WANPIPE: AddressFamily = AddressFamily::WANPIPE;
    pub const LLC: AddressFamily = AddressFamily::LLC;
    pub const IB: AddressFamily = AddressFamily::IB;
    pub const MPLS: AddressFamily = AddressFamily::MPLS;
    pub const CAN: AddressFamily = AddressFamily::CAN;
    pub const TIPC: AddressFamily = AddressFamily::TIPC;
    pub const BLUETOOTH: AddressFamily = AddressFamily::BLUETOOTH;
    pub const IUCV: AddressFamily = AddressFamily::IUCV;
    pub const RXRPC: AddressFamily = AddressFamily::RXRPC;
    pub const ISDN: AddressFamily = AddressFamily::ISDN;
    pub const PHONET: AddressFamily = AddressFamily::PHONET;
    pub const IEEE802154: AddressFamily = AddressFamily::IEEE802154;
    pub const CAIF: AddressFamily = AddressFamily::CAIF;
    pub const ALG: AddressFamily = AddressFamily::ALG;
    pub const NFC: AddressFamily = AddressFamily::NFC;
    pub const VSOCK: AddressFamily = AddressFamily::VSOCK;
    pub const KCM: AddressFamily = AddressFamily::KCM;
    pub const QIPCRTR: AddressFamily = AddressFamily::QIPCRTR;
    pub const SMC: AddressFamily = AddressFamily::SMC;
    pub const MAX: AddressFamily = AddressFamily::MAX;
}

pub struct IfAddrs {
    base: *mut ifaddrs,
}
impl IfAddrs {
    pub fn new() -> Result<Self, ()> {
        let mut ifaddrs = std::ptr::null_mut();
        let num = unsafe { getifaddrs(&mut ifaddrs) };
        return if num == -1 {
            Err(())
        } else {
            Ok(Self { base: ifaddrs })
        };
    }
    pub fn iter<'a>(&'a self) -> IfAddrsIter<'a> {
        IfAddrsIter {
            current: self.base,
            parent: std::marker::PhantomData,
        }
    }
}
impl Drop for IfAddrs {
    fn drop(&mut self) {
        unsafe {
            freeifaddrs(self.base);
        }
    }
}
pub struct IfAddrsIter<'a> {
    current: *mut ifaddrs,
    parent: std::marker::PhantomData<&'a IfAddrs>,
}

impl<'a> std::iter::Iterator for IfAddrsIter<'a> {
    type Item = IfAddr<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current != std::ptr::null_mut() {
            let current = unsafe { self.current.as_mut() }.unwrap();
            self.current = current.ifa_next;
            return Some(IfAddr::new(current));
        }
        return None;
    }
}
pub enum IfAddr<'a> {
    Packet {
        name: &'a str,
        flags: ifa::Flags,
        link_stats: &'a RtnlLinkStats,
    },
    Inet {
        name: &'a str,
        flags: ifa::Flags,
        addr: std::net::IpAddr,
        netmask: std::net::IpAddr,
    },
    Bluetooth {
        name: &'a str,
        flags: ifa::Flags,
        addr: [u8; 6],
        channel: u8,
    },
    Unsupported {
        family: AF::AddressFamily,
        flags: ifa::Flags,
        name: &'a str,
    },
}
impl<'a> IfAddr<'a> {
    fn new(ifaddr: &mut ifaddrs) -> Self {
        let name = unsafe {
            std::ffi::CStr::from_ptr(ifaddr.ifa_name)
                .to_str()
                .unwrap_or("")
        };
        let addr = unsafe { ifaddr.ifa_addr.as_ref().unwrap() };

        return match addr.sa_family {
            AF::INET | AF::INET6 => {
                let (ipaddr, mask) = match addr.sa_family {
                    AF::INET => {
                        let addr: &socketaddr_in = unsafe { std::mem::transmute(addr) };
                        let mask: &socketaddr_in =
                            unsafe { std::mem::transmute(ifaddr.ifa_netmask.as_ref().unwrap()) };

                        (
                            std::net::IpAddr::from(addr.sin_addr),
                            std::net::IpAddr::from(mask.sin_addr),
                        )
                    }
                    AF::INET6 => {
                        let addr: &socketaddr_in6 = unsafe { std::mem::transmute(addr) };
                        let mask: &socketaddr_in6 =
                            unsafe { std::mem::transmute(ifaddr.ifa_netmask.as_ref().unwrap()) };

                        (
                            std::net::IpAddr::from(addr.sin6_addr),
                            std::net::IpAddr::from(mask.sin6_addr),
                        )
                    }
                    _ => {
                        unreachable!()
                    }
                };
                Self::Inet {
                    name: name,
                    flags: ifa::Flags::new(ifaddr.ifa_flags),
                    addr: ipaddr,
                    netmask: mask,
                }
            }
            AF::PACKET => Self::Packet {
                name: name,
                flags: ifa::Flags::new(ifaddr.ifa_flags),
                link_stats: unsafe { (ifaddr.ifa_data as *const RtnlLinkStats).as_ref().unwrap() },
            },
            AF::BLUETOOTH => {
                let addr: &socketaddr_rc = unsafe { std::mem::transmute(addr) };
                Self::Bluetooth {
                    name: name,
                    flags: ifa::Flags::new(ifaddr.ifa_flags),
                    addr: addr.rc_bdaddr,
                    channel: addr.rc_channel,
                }
            }
            _ => Self::Unsupported {
                name: name,
                flags: ifa::Flags::new(ifaddr.ifa_flags),
                family: addr.sa_family,
            },
        };
    }
    pub fn name(&self) -> &str {
        match self {
            &IfAddr::Inet { name, .. } => name,
            &IfAddr::Packet { name, .. } => name,
            &IfAddr::Bluetooth { name, .. } => name,
            &IfAddr::Unsupported { name, .. } => name,
        }
    }
    pub fn is_inet(&self) -> bool {
        match self {
            &IfAddr::Inet { .. } => true,
            _ => false,
        }
    }
    pub fn is_packet(&self) -> bool {
        match self {
            &IfAddr::Packet { .. } => true,
            _ => false,
        }
    }
    pub fn packet(&self) -> Option<&'a RtnlLinkStats> {
        match self {
            &IfAddr::Packet { link_stats, .. } => Some(link_stats),
            _ => None,
        }
    }
    pub fn inet(&self) -> Option<(&std::net::IpAddr, &std::net::IpAddr)> {
        match self {
            &IfAddr::Inet {
                ref addr,
                ref netmask,
                ..
            } => Some((addr, netmask)),
            _ => None,
        }
    }
    pub fn family(&self) -> AF::AddressFamily {
        match self {
            &IfAddr::Inet { ref addr, .. } => {
                if addr.is_ipv4() {
                    AF::INET
                } else {
                    AF::INET6
                }
            }
            &IfAddr::Packet { .. } => AF::PACKET,
            &IfAddr::Bluetooth { .. } => AF::BLUETOOTH,
            &IfAddr::Unsupported { family, .. } => family,
        }
    }
    pub fn flags(&self) -> ifa::Flags {
        match self {
            &IfAddr::Inet { ref flags, .. } => *flags,
            &IfAddr::Packet { ref flags, .. } => *flags,
            &IfAddr::Bluetooth { ref flags, .. } => *flags,
            &IfAddr::Unsupported { ref flags, .. } => *flags,
        }
    }
}
