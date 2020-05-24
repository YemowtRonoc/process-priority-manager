#[path = "windows_process/process.rs"]
mod process;

fn main() {
    let process_id = process::get_process_id_from_name("OVRServer_x64.exe");
    println!("Returned process ID: {:?}", process_id);
}
