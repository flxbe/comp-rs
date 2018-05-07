use std::os::unix::io::AsRawFd;
use super::drm::{open, Color, DeviceInterface};

pub struct GFX {
    di: DeviceInterface,
}

impl GFX {
    pub fn new() -> GFX {
        GFX {
            di: open("/dev/dri/card0")
        }
    }

    pub fn point(&mut self, x: u32, y: u32, c: &Color) {
        self.di.fbs[0].set(x, y, c);
    }

    pub fn vertical_line(&mut self, x: u32, y: u32, height: u32, c: &Color) {
        for l in 0..height {
            self.point(x, y + l, c);
        }
    }

    pub fn horizontal_line(&mut self, x:u32, y:u32, width: u32, c: &Color) {
        for l in 0..width {
            self.point(x + l, y, c);
        }
    }

    pub fn rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, c: &Color) {
        for l in 0..width {
            self.vertical_line(x + l, y, height, c);
        }
    }

    pub fn clear(&mut self) {
        let c = Color::new(255, 255, 255, 255);
        let height = self.di.fbs[0].height();        
        let width = self.di.fbs[0].width();
        self.rectangle(0, 0, width, height, &c);
    }

    pub fn test(&mut self) {
        let c = Color::new(255, 255, 255, 255);
        self.vertical_line(0, 0, 100, &c);
        self.horizontal_line(0, 0, 100, &c);
        self.rectangle(10, 10, 100, 100, &c);
    }
}
