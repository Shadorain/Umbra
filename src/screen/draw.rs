use std::fmt::Display;

#[allow(dead_code)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[allow(dead_code)]
pub struct DrawBuffer<'a> {
    buf: &'a str,
}

impl<'a> DrawBuffer<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self { buf }
    }
}

impl<'a> From<&'a str> for DrawBuffer<'a> {
    fn from(buf: &'a str) -> Self {
        Self { buf }
    }
}

impl<'a> Display for DrawBuffer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.buf)
    }
}
