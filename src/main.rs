#![cfg(windows)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

#[cfg(windows)]
extern crate winapi;

use core::panic::PanicInfo;
use core::mem::{self, MaybeUninit};

use winapi::shared::minwindef::DWORD;
use winapi::um::winuser::*;
use winapi::um::synchapi::Sleep;

/// A translation of the "ScreenSaverStopper" into Rust, original found here:
/// https://github.com/RaymiiOrg/ScreenSaverStopper
///
/// Links that helped me get this written:
/// https://github.com/rust-lang/rust/issues/54137
/// https://w3c.github.io/uievents/tools/key-event-viewer.html
/// https://doc.rust-lang.org/nomicon/panic-handler.html
/// https://github.com/johnthagen/min-sized-rust
/// https://doc.rust-lang.org/1.7.0/book/no-stdlib.html

// linkage to CRT library according to crt-static flag set in .cargo/config
#[cfg(target_feature = "crt-static")]
#[link(name = "libcmt")]
extern {}
#[cfg(not(target_feature = "crt-static"))]
#[link(name = "msvcrt")]
extern {}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn main() -> i32 {
    let sleeptime: DWORD = 40_000;
    let mut ip = INPUT{
        type_: INPUT_KEYBOARD,
        u: unsafe {
            let mut k: INPUT_u = MaybeUninit::uninit().assume_init();
            *k.ki_mut() = KEYBDINPUT{
                wVk: 0x87, // virtual-key cod for "F24"
                wScan: 0, // hardware scan for key
                time: 0,
                dwExtraInfo: 0,
                dwFlags: 0,
            };
            k
        }
    };

    loop { unsafe {
        ip.u.ki_mut().dwFlags = 0;
        let _ = SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
        ip.u.ki_mut().dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
        Sleep(sleeptime);
    }}

    0
}

#[panic_handler]
fn my_panic(_info: &PanicInfo) -> ! {
    loop {}
}