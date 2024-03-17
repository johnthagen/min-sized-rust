#![no_main]

use std::fs::File;
use std::io::Write as _;
use std::os::windows::{io::FromRawHandle as _, raw::HANDLE};

#[link(name = "kernel32")]
extern "system" {
    pub fn GetStdHandle(nstdhandle: u32) -> HANDLE;
}

pub const STD_OUTPUT_HANDLE: u32 = 4294967285;

fn stdout() -> File {
    unsafe { File::from_raw_handle(GetStdHandle(STD_OUTPUT_HANDLE)) }
}

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> u32 {
    let mut stdout = stdout();
    stdout.write_all(b"Hello, world!\n").unwrap();

    0
}
