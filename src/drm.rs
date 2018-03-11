extern crate libc;
extern crate memmap;

use std::{thread, time};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;

use self::libc::ioctl;
use super::drm_const::*;
use super::ffi::*;

use self::memmap::{MmapOptions};

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

pub fn open(path: &str) {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path).unwrap();

    let mut res = drm_mode_card_res::default();
    unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

    /**
     * read device infos
     **/
    let mut fb_ids: Vec<u64> = create_buffer(res.count_fbs);
    let mut crtc_ids: Vec<u64> = create_buffer(res.count_crtcs);
    let mut connector_ids: Vec<u64> = create_buffer(res.count_connectors);
    let mut encoder_ids: Vec<u64> = create_buffer(res.count_encoders);
    res.fb_id_ptr = fb_ids.as_mut_ptr() as u64;
    res.crtc_id_ptr = crtc_ids.as_mut_ptr() as u64;
    res.connector_id_ptr = connector_ids.as_mut_ptr() as u64;
    res.encoder_id_ptr = encoder_ids.as_mut_ptr() as u64;
    unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETRESOURCES, res.as_ptr()) };

    let mut fbs: Vec<FrameBuffer> = Vec::new();
    for &connector_id in connector_ids.iter() {
        /**
         * read connector infos
         **/
        let mut connector = drm_mode_get_connector::default();
        connector.connector_id = connector_id as u32;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETCONNECTOR, connector.as_ptr()) };

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

        /**
         * create the framebuffer
         **/
        let mut buffer_config = drm_mode_create_dumb {
            width: modes[0].hdisplay as u32,
            height: modes[0].vdisplay as u32,
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

        /**
         * mmap the framebuffer
         **/
        let mut map_config = drm_mode_map_dumb::default();
        map_config.handle = buffer_config.handle;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_MAP_DUMB, map_config.as_ptr()) };

        let mut mmap = unsafe {
            MmapOptions::new()
                .offset(map_config.offset as usize)
                .len(buffer_config.size as usize)
                .map_mut(&fd)
                .unwrap()
        };

        mmap[0] = 0xff;
        mmap[1] = 0xff;
        mmap[2] = 0xff;
        mmap[3] = 0xff;
            
        /**
         * initialize the crtc
         **/
        let mut encoder = drm_mode_get_encoder::default();
        encoder.encoder_id = connector.encoder_id;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETENCODER, encoder.as_ptr()) };

        let mut crtc = drm_mode_crtc::default();
        crtc.crtc_id = encoder.crtc_id;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_GETCRTC, crtc.as_ptr()) };

        let mut connectors = vec![connector_id];
        crtc.fb_id = fb_cmd.fb_id;
        crtc.set_connectors_ptr = connectors.as_ptr() as u64;
        crtc.count_connectors = connectors.len() as u32;
        crtc.mode_valid = 1;
        unsafe { ioctl(fd.as_raw_fd(), DRM_IOCTL_MODE_SETCRTC, crtc.as_ptr()) };

        thread::sleep(time::Duration::from_millis(1000));
    }
}

#[derive(Debug)]
struct FrameBuffer {
    fb_cmd: drm_mode_fb_cmd
}

