# Example
```rust
extern crate ifaddrs_rs;
use ifaddrs_rs::*;
fn main(){
    let ifaddrs = match IfAddrs::new(){
        Ok(v)=>v,
        Err( _ )=>{
            panic!("can not get ifaddrs");
        }
    };
    for ifaddr in ifaddrs.iter(){
        match ifaddr{
            IfAddr::Inet { name, flags, addr, netmask }=>{
                println!("{:?} {}",ifaddr.family(), name);
                println!("\t{}", flags);
                println!("\taddress: {}",addr);
                println!("\tmask: {}",netmask);
            },
            IfAddr::Packet { name, flags, link_stats }=>{
                println!("{:?} {}",ifaddr.family(), name);
                println!("\t{}", flags);
                println!("\ttx_packets: {}; rx_packets: {};", link_stats.tx_packets, link_stats.rx_packets);
            },
            IfAddr::Bluetooth { name, flags, addr, channel }=>{
                println!("{:?} {}",ifaddr.family(), name);
                println!("\t{}", flags);
            },
            IfAddr::Unsupported { family, flags, name }=>{
                println!("{:?} {}",family, name);
                println!("\t{}", flags);
            }
        }
    }
}
```