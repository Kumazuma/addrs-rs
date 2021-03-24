#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
const AF_INET:u16 = 2;
const AF_INET6:u16 = 10;
const AF_PACKET:u16 = 17;
#[repr(C)]
struct ifaddrs{
    ifa_next:*mut ifaddrs,
    ifa_name:*const i8,
    ifa_flags: std::os::raw::c_uint,
    ifa_addr:*const socketaddr,
    ifa_netmask:*const socketaddr,
    ifa_ifu:ifa_ifu_t,
    ifa_data:*const std::os::raw::c_void
}
#[repr(C)]
struct socketaddr{
    sa_family: std::os::raw::c_ushort
}
#[repr(C)]
struct socketaddr_in{
    sin_family: std::os::raw::c_ushort,
    sin_port: std::os::raw::c_ushort,
    sin_addr: [u8;4]
}
#[repr(C)]
struct socketaddr_in6{
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: [u8;16]
}
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RtnlLinkStats{
    pub rx_packets:u32,
    pub tx_packets:u32,
    pub rx_bytes:u32,
    pub tx_bytes:u32,
    pub rx_errors:u32,
    pub tx_errors:u32,
    pub rx_dropped:u32,
    pub tx_dropped:u32,
    pub multicast:u32,
    pub collisions:u32,
    pub rx_length_errors:u32,
    pub rx_over_errors:u32,
    pub rx_crc_errors:u32,
    pub rx_frame_errors:u32,
    pub rx_fifo_errors:u32,
    pub rx_missed_errors:u32,
    pub tx_aborted_errors:u32,
    pub tx_carrier_errors:u32,
    pub tx_fifo_errors:u32,
    pub tx_heartbeat_errors:u32,
    pub tx_window_errors:u32,
    pub rx_compressed:u32,
    pub tx_compressed:u32,
}

#[repr(C)]
union ifa_ifu_t{
    ifu_broadadr:*mut socketaddr,
    ifu_dstaddr:*mut socketaddr
}
extern "C"{
    fn getifaddrs(ifap:*mut *mut ifaddrs)->std::os::raw::c_int;
    fn freeifaddrs(ifap:*mut ifaddrs);
}

pub struct IfAddrs{
    base:*mut ifaddrs
}
impl IfAddrs{
    pub fn new()->Result<Self, ()>{
        let mut ifaddrs = std::ptr::null_mut();
        let num = unsafe{
            getifaddrs(&mut ifaddrs)
        };
        return if num == -1{
             Err(())
        }
        else{
            Ok(Self{
                base:ifaddrs
            })
        };
    }
    pub fn iter<'a>(&'a self)->IfAddrsIter<'a>{
        IfAddrsIter{
            current:self.base,
            parent:std::marker::PhantomData
        }
    }
}
impl Drop for IfAddrs{
    fn drop(&mut self){
        unsafe {
            freeifaddrs(self.base);
        }
    }
}
pub struct IfAddrsIter<'a>{
    current:*mut ifaddrs,
    parent:std::marker::PhantomData<&'a IfAddrs>
}

impl<'a> std::iter::Iterator for IfAddrsIter<'a>{
    type Item = IfAddr<'a>;
    fn next(&mut self)->Option<Self::Item>{
        if self.current != std::ptr::null_mut(){
            let current = unsafe{
                self.current.as_mut()
            }.unwrap();
            self.current = current.ifa_next;
            return Some(
                IfAddr::new(current)
            )
        }
        return None;
    }
}
pub enum IfAddr<'a>{
    Packet{
        name:&'a str,
        link_stats:&'a RtnlLinkStats,
    },
    Inet{
        name:&'a str,
        addr:std::net::IpAddr,
        netmask:std::net::IpAddr,
    }
}
impl<'a> IfAddr<'a>{
    fn new(ifaddr:&mut ifaddrs)->Self{
        let name = unsafe{
            std::ffi::CStr::from_ptr(ifaddr.ifa_name).to_str().unwrap_or("")
        };
        let addr = unsafe{
            ifaddr.ifa_addr.as_ref().unwrap()
        };

        return  match addr.sa_family{
            AF_INET | AF_INET6 =>{
                let (ipaddr, mask) = match addr.sa_family{
                    AF_INET=>{
                        let addr:&socketaddr_in = unsafe{std::mem::transmute(addr)};
                        let mask:&socketaddr_in = unsafe{std::mem::transmute(ifaddr.ifa_netmask.as_ref().unwrap())};
                        
                        (std::net::IpAddr::from(addr.sin_addr), std::net::IpAddr::from(mask.sin_addr))

                    },
                    AF_INET6=>{
                        let addr:&socketaddr_in6 = unsafe{std::mem::transmute(addr)};
                        let mask:&socketaddr_in6 = unsafe{std::mem::transmute(ifaddr.ifa_netmask.as_ref().unwrap())};

                        (std::net::IpAddr::from(addr.sin6_addr), std::net::IpAddr::from(mask.sin6_addr))
                    } 
                    _=>{
                        unreachable!()
                    }
                };
                Self::Inet{
                    name:name,
                    addr:ipaddr,
                    netmask:mask
                }
            },
            AF_PACKET=>Self::Packet{
                name:name,
                link_stats:unsafe{(ifaddr.ifa_data as *const RtnlLinkStats).as_ref().unwrap()}
            },
            _=>{
                unreachable!();
            }
        };
    }
    pub fn name(&self)->&str{
        match self{
            &IfAddr::Inet{name, .. }=>name,
            &IfAddr::Packet{name,.. }=>name
        }
    }
    pub fn is_inet(&self)->bool{
        match self{
            &IfAddr::Inet{ .. }=>true,
            &IfAddr::Packet{.. }=>false
        }
    }
    pub fn is_packet(&self)->bool{
        match self{
            &IfAddr::Inet{ .. }=>false,
            &IfAddr::Packet{.. }=>true
        }
    }
    pub fn packet(&self)->Option<&'a RtnlLinkStats>{
        match self{
            &IfAddr::Inet{ .. }=>None,
            &IfAddr::Packet{link_stats , ..}=>Some(link_stats)
        }
    }
    pub fn inet(&self)->Option<(&std::net::IpAddr, &std::net::IpAddr)>{
        match self{
            &IfAddr::Inet{ ref addr,ref netmask, .. }=>Some((addr, netmask)),
            &IfAddr::Packet{ ..}=>None
        }
    }
}