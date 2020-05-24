#[path = "windows_process/process.rs"]
mod process;

fn main() {
    extern crate winapi;
    use winapi::um::winbase::HIGH_PRIORITY_CLASS;

    let process_id =
        process::set_cpu_priority_for_process("OVRServer_x64.exe", HIGH_PRIORITY_CLASS);
    println!("Returned process ID: {:?}", process_id);
}
