use chrono::UTC;
use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    static ref NOW: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(UTC::now().timestamp() as usize));
}

pub fn now() -> i64 {
    NOW.load(Ordering::Relaxed) as i64
}

pub fn update_time() {
    let dur = time::Duration::from_millis(500);
    loop {
        thread::sleep(dur);
        let now = UTC::now().timestamp() as usize;
        let order = Ordering::Relaxed;
        NOW.store(now, order);
    }
}

#[inline]
pub fn sleep_hertz(hertz: u16) {
    let one_sec: f64 = 1_000_000_000.0;
    let delay = time::Duration::new(0, (one_sec / (hertz as f64)) as u32);
    thread::sleep(delay);
}
