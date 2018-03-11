#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


#![allow(unused)]
mod ffi;
mod drm_const;
mod drm;

fn main() {
    drm::open("/dev/dri/card0");
}
