use std::io::Error;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
fn windows_message_box(msg: &str) -> i32 {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_OK};

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();

    let ret: i32 = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK) };
    ret
}

#[path = "windows_process/process.rs"]
mod process;

fn main() {
    // println!("Hello, world!");

    let result = windows_message_box("This is Conor's message");
    if result == 0 {
        println!(
            "Error calling windows_message_box: {:?}",
            Error::last_os_error()
        );
    }

    let process_id = process::get_process_id_from_name("OVRServer_x64.exe");
    println!("Returned process ID: {:?}", process_id);
}
