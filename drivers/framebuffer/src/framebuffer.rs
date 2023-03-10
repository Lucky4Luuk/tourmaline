#[derive(Copy, Clone)]
pub enum PixelFormat {
    Rgb,
    Bgr,
    U8
}

#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub bytes_per_pixel: usize,
    pub pixel_format: PixelFormat,
}

pub struct Framebuffer {
    fb: &'static mut [u8],
    info: FramebufferInfo,
}

impl Framebuffer {
    pub fn new(fb: &'static mut [u8], info: FramebufferInfo) -> Self {
        Self {
            fb,
            info,
        }
    }

    pub fn width(&self) -> usize { self.info.width }
    pub fn height(&self) -> usize { self.info.height }
    pub fn info(&self) -> FramebufferInfo { self.info }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> [u8; 3] {
        if x >= self.width() || y >= self.height() { return [0,0,0]; }
        let byte_idx = x * self.info.bytes_per_pixel + y * self.info.stride;
        let next_byte_idx = byte_idx + self.info.bytes_per_pixel;
        let raw_pixel = &mut self.fb[byte_idx..next_byte_idx];
        let c = &raw_pixel[..3];
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                [c[0], c[1], c[2]]
            },
            PixelFormat::Bgr => {
                [c[2], c[1], c[0]]
            },
            PixelFormat::U8 => {
                [c[0], 0, 0]
            },
            _ => unimplemented!(),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if x >= self.width() || y >= self.height() { return; }
        let byte_idx = x * self.info.bytes_per_pixel + y * self.info.stride;
        let next_byte_idx = byte_idx + self.info.bytes_per_pixel;
        let raw_pixel = &mut self.fb[byte_idx..next_byte_idx];
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                raw_pixel[..3].copy_from_slice(&color);
            },
            PixelFormat::Bgr => {
                let color = [color[2], color[1], color[0]];
                raw_pixel[..3].copy_from_slice(&color);
            },
            PixelFormat::U8 => {
                raw_pixel[0] = color[0];
            },
            _ => unimplemented!(),
        }
    }

    pub fn for_pixel_in_range<F: Fn(usize, usize, usize, usize, &mut [u8; 3])>(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, f: F) {
        let x1 = x1.min(self.width());
        let y1 = y1.min(self.height());
        let w = x1 - x0;
        let h = y1 - y0;
        for x in x0..x1 {
            for y in y0..y1 {
                let mut pixel = self.get_pixel(x,y);
                f(x - x0, y - y0, w, h, &mut pixel);
                self.set_pixel(x,y,pixel);
            }
        }
    }

    pub fn for_pixel<F: Fn(usize, usize, usize, usize, &mut [u8; 3])>(&mut self, f: F) {
        self.for_pixel_in_range(0,0, self.info.width,self.info.height, f)
    }
}
