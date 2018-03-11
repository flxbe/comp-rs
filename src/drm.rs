extern crate libc;

use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;

use self::libc::ioctl;
use super::drm_const::*;
use super::ffi::*;

pub trait Pointer {
    fn as_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}

impl Pointer for drm_mode_card_res {}
impl Pointer for drm_mode_get_connector {}
impl Pointer for drm_mode_create_dumb {}
impl Pointer for drm_mode_fb_cmd {}
impl Pointer for drm_mode_map_dumb {}
impl Pointer for drm_mode_get_encoder {}
impl Pointer for drm_mode_crtc {}

fn create_buffer<T>(size: u32) -> Vec<T> {
    let mut buffer: Vec<T> = Vec::with_capacity(size as usize);
    unsafe { buffer.set_len(size as usize) };
    buffer
}

#[derive(Debug)]
pub struct Device {
    fd: File,
    connectors: Vec<Connector>,
    _drm_mode_card_res: drm_mode_card_res
}

impl Device {
    pub fn open(path: &str) -> Device {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path).unwrap();

        let mut res = drm_mode_card_res::default();
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

        // create buffers and read device infos
        let mut fb_ids: Vec<u64> = create_buffer(res.count_fbs);
        let mut crtc_ids: Vec<u64> = create_buffer(res.count_crtcs);
        let mut connector_ids: Vec<u64> = create_buffer(res.count_connectors);
        let mut encoder_ids: Vec<u64> = create_buffer(res.count_encoders);
        res.fb_id_ptr = fb_ids.as_mut_ptr() as u64;
        res.crtc_id_ptr = crtc_ids.as_mut_ptr() as u64;
        res.connector_id_ptr = connector_ids.as_mut_ptr() as u64;
        res.encoder_id_ptr = encoder_ids.as_mut_ptr() as u64;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

        let mut connectors: Vec<Connector> = connector_ids.iter().map(|&id| Connector::new(&fd, id)).collect();
        for connector in connectors.iter_mut() {
            connector.create_fb(&fd);
        }

        return Device {
            fd,
            connectors,
            _drm_mode_card_res: res
        }
    }

    fn set_master(&mut self) {
        unsafe { ioctl(self.fd.as_raw_fd(), DRM_IOCTL_SET_MASTER, 0) };
    }
}

#[derive(Debug)]
pub struct Connector {
    modes: Vec<drm_mode_modeinfo>,
    props: Vec<u64>,
    prop_values: Vec<u64>,
    encoders: Vec<u64>,

    fb: Option<FrameBuffer>,
    encoder: Encoder,

    _connector: drm_mode_get_connector
}

impl Connector {
    pub fn new(fd: &File, id: u64) -> Connector {
        println!("inspect connector with id {}", id);

        let mut connector = drm_mode_get_connector::default();
        connector.connector_id = id as u32;

        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETCONNECTOR, connector.as_ptr()) };

        println!("total modes: {}", connector.count_modes);
        println!("total props: {}", connector.count_props);
        println!("total encoders: {}", connector.count_encoders);

        let mut modes: Vec<drm_mode_modeinfo> = create_buffer(connector.count_modes);
        let mut props: Vec<u64> = create_buffer(connector.count_props);
        let mut prop_values: Vec<u64> = create_buffer(connector.count_props);
        let mut encoders: Vec<u64> = create_buffer(connector.count_encoders);

        connector.modes_ptr = modes.as_mut_ptr() as u64;
        connector.props_ptr = props.as_mut_ptr() as u64;
        connector.prop_values_ptr = prop_values.as_mut_ptr() as u64;
        connector.encoders_ptr = encoders.as_mut_ptr() as u64;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETCONNECTOR, connector.as_ptr()) };

        for mode in modes.iter() {
            let name = mode.name
                .iter()
                .map(|&c| c as u8 as char)
                .collect::<String>();
            println!("{}", name);
        }

        let encoder = Encoder::new(fd, connector.encoder_id);

        Connector {
            modes,
            props,
            prop_values,
            encoders,

            fb: None,
            encoder,

            _connector: connector
        }
    }

    pub fn create_fb(&mut self, fd: &File) {
        let mut buffer_config = drm_mode_create_dumb {
            width: self.modes[0].hdisplay as u32,
            height: self.modes[0].vdisplay as u32,
            bpp: 32,
            flags: 0,
            pitch: 0,
            size: 0,
            handle: 0
        };
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_CREATE_DUMB, buffer_config.as_ptr()) };

        let mut fb_cmd = drm_mode_fb_cmd {
            fb_id: 0,
            width: buffer_config.width,
            height: buffer_config.height,
            bpp: buffer_config.bpp,
            pitch: buffer_config.pitch,
            depth: 24,
            handle: buffer_config.handle
        };
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_ADDFB, fb_cmd.as_ptr()) };

        let mut map_config = drm_mode_map_dumb::default();
        map_config.handle = buffer_config.handle;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_MAP_DUMB, map_config.as_ptr()) };

        let mut connectors = vec![self._connector.connector_id];
        let mut crtc = &mut self.encoder._crtc;
        crtc.fb_id = fb_cmd.fb_id;
        crtc.set_connectors_ptr = connectors.as_ptr() as u64;
        crtc.count_connectors = connectors.len() as u32;
    }
}

#[derive(Debug)]
struct Encoder {
    _encoder: drm_mode_get_encoder,
    _crtc: drm_mode_crtc
}

impl Encoder {
    pub fn new(fd: &File, id: u32) -> Encoder {
        println!("inspect encoder with id {}", id);

        let mut encoder = drm_mode_get_encoder::default();
        encoder.encoder_id = id;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETENCODER, encoder.as_ptr()) };

        println!("inspect crtc with id {}", encoder.crtc_id);
        let mut crtc = drm_mode_crtc::default();
        crtc.crtc_id = encoder.crtc_id;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETCRTC, crtc.as_ptr()) };

        Encoder {
            _encoder: encoder,
            _crtc: crtc
        }
    }
}

pub fn open(path: &str) -> Device {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path).unwrap();

    let mut res = drm_mode_card_res::default();
    unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

    // create buffers and read device infos
    let mut fb_ids: Vec<u64> = create_buffer(res.count_fbs);
    let mut crtc_ids: Vec<u64> = create_buffer(res.count_crtcs);
    let mut connector_ids: Vec<u64> = create_buffer(res.count_connectors);
    let mut encoder_ids: Vec<u64> = create_buffer(res.count_encoders);
    res.fb_id_ptr = fb_ids.as_mut_ptr() as u64;
    res.crtc_id_ptr = crtc_ids.as_mut_ptr() as u64;
    res.connector_id_ptr = connector_ids.as_mut_ptr() as u64;
    res.encoder_id_ptr = encoder_ids.as_mut_ptr() as u64;
    unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

    let mut connectors: Vec<Connector> = connector_ids.iter().map(|&id| Connector::new(&fd, id)).collect();
    for connector in connectors.iter_mut() {
        connector.create_fb(&fd);
    }

    return Device {
        fd,
        connectors,
        _drm_mode_card_res: res
    }
}


#[derive(Debug)]
struct FrameBuffer {
    ptr: u64,
    width: u32,
    height: u32
}

