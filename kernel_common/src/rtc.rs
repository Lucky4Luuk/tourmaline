use conquer_once::spin::OnceCell;
use crate::Mutex;
use cmos_rtc::{ReadRTC, Time};

static RTC: OnceCell<Mutex<ReadRTC>> = OnceCell::uninit();

fn get_cmos() -> &'static Mutex<ReadRTC> {
    RTC.get_or_init(|| Mutex::new(ReadRTC::new(0x00, 0x00)))
}

pub fn rtc_time() -> Time {
    get_cmos().lock().read()
}

pub fn rtc_time_seconds() -> u32 {
    fn time_to_sec(time: Time) -> u32 {
        time.second as u32 + (time.minute as u32) * 60 + (time.hour as u32) * 60 * 60 + (time.day as u32) * 60 * 60 * 24
    }
    time_to_sec(rtc_time())
}

/// Very simple sleep function based on the RTC.
/// It does not read anything more accurate than seconds, so you can only sleep for
/// whole seconds.
/// This function can also break when it runs right over the month/year/century barrier.
/// It does not take into account the current month, year or century.
pub fn rtc_sleep(seconds: u32) {
    let start_sec = rtc_time_seconds();
    'wait: loop {
        let now_sec = rtc_time_seconds();
        if now_sec > start_sec + seconds {
            break 'wait;
        }
    }
}
