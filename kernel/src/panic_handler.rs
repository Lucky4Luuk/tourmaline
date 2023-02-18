use core::panic::PanicInfo;
use crate::{framebuffer, util};

/// This function is called on panic.
/// It uses the framebuffer to show a nice looking panic graphic.
/// If the framebuffer fails to initialize, it'll panic, and this
/// will be broken!
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let bg_col = [39, 92, 53];
    let fg_col = [255; 3];

    let mut buf = [0u8; 1024];

    let fb = framebuffer::fb_mut();
    let width = fb.width();
    let height = fb.height();
    fb.set_clear_color(bg_col);
    fb.clear();
    let mut panic_area = framebuffer::Rect::new(0, height / 4, width, height);
    let (panic_width, _) = fb.print(&panic_area, bg_col, true, "PANIC!");
    fb.clear();
    panic_area.x0 = width / 2 - panic_width / 2;
    let (_, delta_height) = fb.print(&panic_area, fg_col, true, "PANIC!\n");
    let mut text_area = framebuffer::Rect::new(width / 6, height / 4 + delta_height + 8, width * 5 / 6, height * 3 / 4);
    let (_, delta_height) = fb.print(&text_area, fg_col, false, util::show(&mut buf, format_args!("{}\n", info)).unwrap_or("FMT FAILED\n"));
    text_area.y1 = text_area.y1.max(text_area.y0 + delta_height);
    let outline_area = framebuffer::Rect::new(text_area.x0 - 2, text_area.y0 - 2, text_area.x1 + 2, text_area.y1 + 2);
    fb.outline_double(&outline_area, fg_col);
    loop {}
}
