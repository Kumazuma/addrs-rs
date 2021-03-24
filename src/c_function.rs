use super::c_type::*;
extern "C" {
    pub(crate) fn getifaddrs(ifap: *mut *mut ifaddrs) -> std::os::raw::c_int;
    pub(crate) fn freeifaddrs(ifap: *mut ifaddrs);
}
