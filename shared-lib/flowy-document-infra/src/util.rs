use lib_ot::core::{NEW_LINE, WHITESPACE};
use std::sync::atomic::{AtomicI64, Ordering::SeqCst};

#[inline]
pub fn find_newline(s: &str) -> Option<usize> {
    match s.find(NEW_LINE) {
        None => None,
        Some(line_break) => Some(line_break),
    }
}

#[inline]
pub fn is_newline(s: &str) -> bool { s == NEW_LINE }

#[inline]
pub fn is_whitespace(s: &str) -> bool { s == WHITESPACE }

#[inline]
pub fn contain_newline(s: &str) -> bool { s.contains(NEW_LINE) }

#[inline]
pub fn md5<T: AsRef<[u8]>>(data: T) -> String {
    let md5 = format!("{:x}", md5::compute(data));
    md5
}

#[derive(Debug)]
pub struct RevIdCounter(pub AtomicI64);

impl RevIdCounter {
    pub fn new(n: i64) -> Self { Self(AtomicI64::new(n)) }
    pub fn next(&self) -> i64 {
        let _ = self.0.fetch_add(1, SeqCst);
        self.value()
    }
    pub fn value(&self) -> i64 { self.0.load(SeqCst) }

    pub fn set(&self, n: i64) { let _ = self.0.fetch_update(SeqCst, SeqCst, |_| Some(n)); }
}
