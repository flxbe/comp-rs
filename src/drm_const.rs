#![allow(dead_code)]

use std::os::raw::c_ulong;

pub const DRM_IOCTL_SET_MASTER: c_ulong = 0x0000641e;
pub const DRM_IOCTL_MODE_GETRESOURCES: c_ulong = 0xc04064a0;
pub const DRM_IOCTL_MODE_GETCONNECTOR: c_ulong = 0xc05064a7;
pub const DRM_IOCTL_MODE_GETENCODER: c_ulong = 0xc01464a6;
pub const DRM_IOCTL_MODE_GETCRTC: c_ulong = 0xc06864a1;
pub const DRM_IOCTL_MODE_SETCRTC: c_ulong = 0xc06864a2;
pub const DRM_IOCTL_MODE_CREATE_DUMB: c_ulong = 0xc02064b2;
pub const DRM_IOCTL_MODE_ADDFB: c_ulong = 0xc01c64ae;
pub const DRM_IOCTL_MODE_MAP_DUMB: c_ulong = 0xc01064b3;
