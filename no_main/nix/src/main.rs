#![no_main]

use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

fn stdout() -> File {
    unsafe { File::from_raw_fd(1) }
}

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    let mut stdout = stdout();
    stdout.write(b"Hello, world!\n").unwrap();
}
