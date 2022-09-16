use unmem::{DeOwn, AsCStr, cstr};
use std::io::{Write, Read};

mod imp {
    extern "C" {
        pub fn puts(src: super::cstr) -> i32;
    }
}

#[allow(unused_must_use)]
pub fn sout(from: &str) {
    std::io::stdout()
        .lock()
        .write(from.as_bytes());
}

#[allow(unused_must_use)]
pub fn sout_fmt(from: core::fmt::Arguments) {
    std::io::stdout()
        .lock()
        .write_fmt(from);
}

pub fn sin<'a>() -> &'a str {
    let mut i = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut i) {
        ""
    } else {
        i.leak().trim_end()
    }
}

pub macro writef($($args:tt)*) {
    $crate::heart::io::sout_fmt(format_args!($($args)*))
}

#[macro_export]
macro_rules! read {
    () => {
        $crate::heart::io::sin()
    };

    ($type:ty) => {
        $crate::heart::io::sin().parse::<$type>()
    }
}

pub fn puts(src: &str) {
    unsafe {
        imp::puts(cstr::from_str_unchecked(format!("{src}\0").leak()));
    }
}

pub fn cputs(src: &str) {
    unsafe {
        imp::puts(cstr::from_str_unchecked(src));
    }
}

pub macro putf($($body:tt)*) {
    unsafe {
        let mut string = format!($($body)*);
        string.push('\0');
        $crate::heart::io::puts_unchecked(string.leak());
    }
}