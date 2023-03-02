use limine::*;
use kernel_common::framebuffer::PixelFormat;

pub static mut FRAMEBUFFER: Option<FbWrapper> = None;
static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

pub fn init() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        let framebuffer = &framebuffer_response.framebuffers()[0];
        unsafe {
            FRAMEBUFFER = Some(FbWrapper::new(&framebuffer));
        }
    } else {
        panic!("Failed to initialize framebuffer!");
    }
}

#[inline]
pub fn fb_mut() -> &'static mut FbWrapper {
    unsafe {
        FRAMEBUFFER.as_mut().unwrap()
    }
}

pub struct Rect {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl Rect {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TextSize {
    Small,
    Normal,
    Big,
}

#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    width: usize,
    height: usize,
    stride: usize,
    bytes_per_pixel: usize,
    pixel_format: PixelFormat,
}

pub struct FbWrapper {
    fb: &'static mut [u8],
    info: FramebufferInfo,
    clear_color: [u8; 3],
}

impl FbWrapper {
    fn new(fb: &LimineFramebuffer) -> Self {
        let rgb_or_bgr = if fb.memory_model == 1 { PixelFormat::Rgb } else { PixelFormat::Bgr };
        // fb.bpp is bits per pixel
        let (bytes_per_pixel, pixel_format) = match fb.bpp {
            8 => (1, PixelFormat::U8),
            16 => (2, PixelFormat::U8),
            24 => (3, rgb_or_bgr),
            32 => (4, rgb_or_bgr),
            _ => unimplemented!(),
        };
        let info = FramebufferInfo {
            width: fb.width as usize,
            height: fb.height as usize,
            stride: fb.pitch as usize,
            bytes_per_pixel,
            pixel_format,
        };
        let fb_len = (info.width + info.height * info.stride) * info.bytes_per_pixel;
        // let fb_len = fb.size();
        Self {
            fb: unsafe { core::slice::from_raw_parts_mut(fb.address.as_ptr().unwrap(), fb_len) },
            info,
            clear_color: [0,0,0],
        }
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.fb
    }

    pub fn set_clear_color(&mut self, color: [u8; 3]) {
        self.clear_color = color;
    }

    pub fn clear(&mut self) {
        let color = self.clear_color;
        self.for_pixel(|_,_,_,_, pixel| *pixel = color);
    }

    pub fn width(&self) -> usize { self.info.width }
    pub fn height(&self) -> usize { self.info.height }
    pub fn info(&self) -> FramebufferInfo { self.info }

    // TODO: Use bottom of area to stop printing
    pub fn print(&mut self, area: &Rect, color: [u8; 3], size: TextSize, text: &str) -> (usize, usize) {
        use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

        let mut x = area.x0;
        let mut y = area.y0;

        let (size, size_num) = match size {
            TextSize::Small => (RasterHeight::Size16, 16),
            TextSize::Normal => (RasterHeight::Size20, 20),
            TextSize::Big => (RasterHeight::Size32, 32),
        };

        let mut delta_height = 0;

        for c in text.chars() {
            match c {
                '\n' => {
                    x = area.x0;
                    y += size_num;
                    delta_height += size_num;
                },
                '\t' => {
                    x += get_raster_width(FontWeight::Regular, size) * 4;
                },
                _ => {
                    let width = get_raster_width(FontWeight::Regular, size);

                    if x + width > area.x1 {
                        x = area.x0;
                        y += size_num;
                        delta_height += size_num;
                    }

                    let char_raster = get_raster(c, FontWeight::Regular, size).expect("unsupported char");
                    let raster = char_raster.raster();

                    self.for_pixel_in_range(x, y, x + width, y + char_raster.height(), |x,y,w,h, pixel| {
                        let byte = raster[y][x];
                        let effect = 1.0 - (byte as f32 / 255f32);
                        for i in 0..3 {
                            pixel[i] = (pixel[i] as f32 * effect) as u8 + (color[i] as f32 * (1.0 - effect)) as u8;
                        }
                    });

                    x += width;
                }
            }
        }

        (x, delta_height)
    }

    pub fn outline(&mut self, area: &Rect, color: [u8; 3]) {
        for x in area.x0..area.x1 {
            self.set_pixel(x, area.y0, color);
            self.set_pixel(x, area.y1-1, color);
        }

        for y in area.y0..area.y1 {
            self.set_pixel(area.x0, y, color);
            self.set_pixel(area.x1-1, y, color);
        }
    }

    pub fn outline_double(&mut self, area: &Rect, color: [u8; 3]) {
        let area_small = Rect::new(area.x0+1, area.y0+1, area.x1-1, area.y1-1);
        let area_large = Rect::new(area.x0-1, area.y0-1, area.x1+1, area.y1+1);
        self.outline(&area_small, color);
        self.outline(&area_large, color);
    }

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
