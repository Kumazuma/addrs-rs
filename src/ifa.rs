#[derive(Debug, Clone, Copy)]
pub struct Flags(u32);
use std::fmt::{self, *};

use super::c_type::IFF;
impl Display for Flags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut fmted_str = String::with_capacity(1024);
        fmted_str.push('{');
        for it in &IFF::IFF_ENUMS {
            if self.has(it.0) {
                fmted_str.push_str(it.1);
                fmted_str.push_str(", ");
            }
        }
        fmted_str.pop();
        fmted_str.pop();
        fmted_str.push('}');
        f.write_str(&fmted_str)?;
        return Ok(());
    }
}
impl Flags {
    pub(crate) fn new(flags: u32) -> Self {
        Self(flags)
    }
    fn has(&self, flag: u16) -> bool {
        (self.0 & flag as u32) != 0
    }
    pub fn up(&self) -> bool {
        self.has(IFF::UP)
    }
    pub fn broadcast(&self) -> bool {
        self.has(IFF::BROADCAST)
    }
    pub fn debug(&self) -> bool {
        self.has(IFF::DEBUG)
    }
    pub fn loopback(&self) -> bool {
        self.has(IFF::LOOPBACK)
    }
    pub fn point_to_point(&self) -> bool {
        self.has(IFF::POINTOPOINT)
    }
    pub fn notrailers(&self) -> bool {
        self.has(IFF::NOTRAILERS)
    }
    pub fn running(&self) -> bool {
        self.has(IFF::RUNNING)
    }
    pub fn noarp(&self) -> bool {
        self.has(IFF::NOARP)
    }
    pub fn promisc(&self) -> bool {
        self.has(IFF::PROMISC)
    }
    pub fn allmulti(&self) -> bool {
        self.has(IFF::ALLMULTI)
    }
    pub fn master(&self) -> bool {
        self.has(IFF::MASTER)
    }
    pub fn slave(&self) -> bool {
        self.has(IFF::SLAVE)
    }
    pub fn multicast(&self) -> bool {
        self.has(IFF::MULTICAST)
    }
    pub fn portsel(&self) -> bool {
        self.has(IFF::PORTSEL)
    }
    pub fn automedia(&self) -> bool {
        self.has(IFF::AUTOMEDIA)
    }
    pub fn dynamic(&self) -> bool {
        self.has(IFF::DYNAMIC)
    }
}
