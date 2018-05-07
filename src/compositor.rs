use super::drm::Color;
use super::gfx::GFX;

pub struct Compositor {
    gfx: GFX,
    windows: Vec<Window>,
}

impl Compositor {
    pub fn new() -> Compositor {
        Compositor {
            gfx: GFX::new(),
            windows: Vec::new()
        }
    }

    pub fn add_window(&mut self) {
        self.windows.push(Window {
            x: 20,
            y: 20,
            width: 500,
            height: 300
        });
    }

    pub fn render(&mut self) {
        for window in self.windows.iter_mut() {
            window.render(&mut self.gfx);
        }
    }
}

struct Window {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl Window {
    pub fn render(&mut self, gfx: &mut GFX) {
        let top_width = 25;
        let border_width = 1;
        let c = Color::new(255, 255, 255, 255);
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;
    
        gfx.rectangle(x, y, width, 25, &c);
        gfx.rectangle(x, y + top_width, border_width, height - top_width, &c);
        gfx.rectangle(x + width - border_width, y + top_width, border_width, height - top_width, &c);
        gfx.rectangle(x, y + height - border_width, width, border_width, &c);
        // TODO: render window content

    }
}
