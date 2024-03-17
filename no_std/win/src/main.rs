#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use core::ffi::c_void;
use core::panic::PanicInfo;

use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::WriteConsoleA;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Threading::ExitProcess;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
fn mainCRTStartup() -> ! {
    let message = "Hello, world!\n";
    unsafe {
        let console = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteConsoleA(
            console,
            message.as_ptr().cast::<c_void>(),
            message.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null(),
        );

        ExitProcess(0)
    }
}
