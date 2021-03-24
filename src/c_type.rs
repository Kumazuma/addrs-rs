use super::AF;
#[repr(C)]
pub(crate) struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *const i8,
    pub ifa_flags: std::os::raw::c_uint,
    pub ifa_addr: *const socketaddr,
    pub ifa_netmask: *const socketaddr,
    pub ifa_ifu: ifa_ifu_t,
    pub ifa_data: *const std::os::raw::c_void,
}
#[repr(C)]
pub(crate) struct socketaddr {
    pub sa_family: AF::AddressFamily,
}
#[repr(C)]
pub(crate) struct socketaddr_in {
    pub sin_family: AF::AddressFamily,
    pub sin_port: std::os::raw::c_ushort,
    pub sin_addr: [u8; 4],
}
#[repr(C)]
pub(crate) struct socketaddr_in6 {
    pub sin6_family: AF::AddressFamily,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
}
//bluetooth
#[repr(C)]
pub(crate) struct socketaddr_rc {
    pub rc_family: AF::AddressFamily,
    pub rc_bdaddr: [u8; 6],
    pub rc_channel: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RtnlLinkStats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_errors: u32,
    pub tx_errors: u32,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub multicast: u32,
    pub collisions: u32,
    pub rx_length_errors: u32,
    pub rx_over_errors: u32,
    pub rx_crc_errors: u32,
    pub rx_frame_errors: u32,
    pub rx_fifo_errors: u32,
    pub rx_missed_errors: u32,
    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,
    pub rx_compressed: u32,
    pub tx_compressed: u32,
}

#[repr(C)]
pub(crate) union ifa_ifu_t {
    ifu_broadadr: *mut socketaddr,
    ifu_dstaddr: *mut socketaddr,
}

pub(crate) mod IFF {

    pub const UP: u16 = 0x01;
    pub const BROADCAST: u16 = 0x02;
    pub const DEBUG: u16 = 0x04;
    pub const LOOPBACK: u16 = 0x08;
    pub const POINTOPOINT: u16 = 0x10;
    pub const NOTRAILERS: u16 = 0x20;
    pub const RUNNING: u16 = 0x40; /* Resources allocated.  */
    pub const NOARP: u16 = 0x80; /* No address resolution protocol.  */
    pub const PROMISC: u16 = 0x100; /* Receive all packets.  */
    /* Not supported */
    pub const ALLMULTI: u16 = 0x200; /* Receive all multicast packets.  */
    pub const MASTER: u16 = 0x400; /* Master of a load balancer.  */
    pub const SLAVE: u16 = 0x800; /* Slave of a load balancer.  */
    pub const MULTICAST: u16 = 0x1000; /* Supports multicast.  */
    pub const PORTSEL: u16 = 0x2000; /* Can set media type.  */
    pub const AUTOMEDIA: u16 = 0x4000; /* Auto media select active.  */
    pub const DYNAMIC: u16 = 0x8000; /* Dialup device with changing addresses.  */
    pub(crate) const IFF_ENUMS: [(u16, &'static str); 16] = [
        (UP, "UP"),
        (BROADCAST, "BROADCAST"),
        (DEBUG, "DEBUG"),
        (LOOPBACK, "LOOPBACK"),
        (POINTOPOINT, "POINTOPOINT"),
        (NOTRAILERS, "NOTRAILERS"),
        (RUNNING, "RUNNING"),
        (NOARP, "NOARP"),
        (PROMISC, "PROMISC"),
        (ALLMULTI, "ALLMULTI"),
        (MASTER, "MASTER"),
        (SLAVE, "SLAVE"),
        (MULTICAST, "MULTICAST"),
        (PORTSEL, "PORTSEL"),
        (AUTOMEDIA, "AUTOMEDIA"),
        (DYNAMIC, "DYNAMIC"),
    ];
}
