#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused)]
mod ffi;
mod drm_const;
mod drm;
mod gfx;
mod compositor;

use std::{thread, time};
use gfx::GFX;
use drm::Color;
use compositor::Compositor;

use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::{cmp, mem, slice};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct InputEvent {
    button: u8,
    dx: i8,
    dy: i8,
}

#[derive(Debug, Default)]
struct MouseState {
    x: u32,
    y: u32,
    left_down: bool,
    middle_down: bool,
    right_down: bool
}

fn start_compositor() {
    let mut c = Compositor::new();
    c.add_window();
    c.render();

    thread::sleep(time::Duration::from_millis(1000));
}

fn read_mouse() {
    let mut g = GFX::new();
    let c = Color::new(0, 0, 255, 255);
    g.clear();
    g.point(20, 20, &c);

    let mut fd = OpenOptions::new()
        .read(true)
        .open("/dev/input/mouse0").unwrap();

    let mut mouse_state = MouseState::default();
    mouse_state.x = 10;
    mouse_state.y = 10;
    g.point(30, 30, &c);

    let event_size = mem::size_of::<InputEvent>();
    let mut event = InputEvent::default();
    let mut buffer = &mut event as *mut InputEvent as *mut u8;
    let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, event_size) };
    loop {
        g.point(40, 40, &c);
        fd.read(buffer).unwrap(); 
        g.point(50, 50, &c);
        mouse_state.x = cmp::max(0, mouse_state.x as i32 + event.dx as i32) as u32;
        mouse_state.y = cmp::max(0, mouse_state.y as i32 + event.dy as i32) as u32;
        mouse_state.left_down = event.button & 0b100 != 0;
        mouse_state.middle_down = event.button & 0b010 != 0;
        mouse_state.right_down = event.button & 0b001 != 0;
        println!("{:?}", mouse_state);
        //g.clear();
        let c = Color::new(0, 0, 255, 255);
        //g.point(300, 300, &c);
        g.point(mouse_state.x, mouse_state.y, &c);
    }
}

fn main() {
    //start_compositor();
    read_mouse();
}
