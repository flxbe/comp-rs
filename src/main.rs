#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#![allow(unused)]
mod ffi;
mod drm_const;
mod drm;

use drm::Device;

fn main() {
    let mut dev = Device::open("/dev/dri/card0");
}
