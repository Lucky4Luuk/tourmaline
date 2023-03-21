//! A basic delay future, to yield for x amount of time.
//! Currently relies on the RTC timer, which can only
//! measure in full seconds.
// TODO: Once we have a better timer, upgrade this

use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    }
};

use crate::rtc::rtc_time_seconds;

struct DelayFuture {
    finish: u32, // In seconds
}

impl Future for DelayFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if rtc_time_seconds() > self.finish {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[inline]
pub async fn delay(sec: u32) {
    let start = rtc_time_seconds();
    DelayFuture {
        finish: start + sec,
    }.await
}
