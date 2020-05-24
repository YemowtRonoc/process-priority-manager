extern crate winapi;
use winapi::um::tlhelp32::PROCESSENTRY32;
use winapi::um::winnt::HANDLE;

struct WindowsProcesses<'a> {
    snapshot: HANDLE,
    entry: &'a mut PROCESSENTRY32,
}
impl<'a> WindowsProcesses<'a> {
    #[cfg(windows)]
    fn get_pid_from_process_entry(&self) -> u32 {
        use winapi::shared::minwindef::FALSE;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::{GetProcessId, OpenProcess};
        use winapi::um::winnt::PROCESS_ALL_ACCESS;

        let process_handle =
            unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, self.entry.th32ProcessID) };

        // Do stuff..
        let pid = unsafe { GetProcessId(process_handle) };

        unsafe {
            CloseHandle(process_handle);
        };

        pid as u32
    }

    #[cfg(windows)]
    fn windows_process_32_first(&mut self) -> bool {
        use winapi::shared::minwindef::TRUE;
        use winapi::um::tlhelp32::Process32First;

        let result = unsafe { Process32First(self.snapshot, self.entry) };

        result == TRUE
    }

    #[cfg(windows)]
    fn windows_process_32_next(&mut self) -> bool {
        use winapi::shared::minwindef::TRUE;
        use winapi::um::tlhelp32::Process32Next;

        let result = unsafe { Process32Next(self.snapshot, self.entry) };

        result == TRUE
    }

    #[cfg(windows)]
    fn windows_create_toolhelp_32_snapshot(&self) -> HANDLE {
        use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS};

        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
    }

    fn windows_close_handle(&self) -> i32 {
        use winapi::um::handleapi::CloseHandle;

        unsafe { CloseHandle(self.snapshot) }
    }

    pub fn get_pid_from_process_name(&mut self, process_name: &str) -> u32 {
        use std::ffi::CStr;
        self.snapshot = self.windows_create_toolhelp_32_snapshot();
        let mut pid = 0;

        if self.windows_process_32_first() {
            while self.windows_process_32_next() {
                let current_process_name = unsafe { CStr::from_ptr(self.entry.szExeFile.as_ptr()) };
                if current_process_name.to_str().unwrap() == process_name {
                    pid = self.get_pid_from_process_entry();
                    break;
                }
            }
        }
        self.windows_close_handle();

        pid as u32
    }
}

fn windows_initialise_process_entry() -> PROCESSENTRY32 {
    use std::mem::size_of;

    let process_entry_size = size_of::<PROCESSENTRY32>();
    let entry = PROCESSENTRY32 {
        cntThreads: 0,
        cntUsage: 0,
        dwFlags: 0,
        dwSize: process_entry_size as u32,
        pcPriClassBase: 0,
        szExeFile: [0; 260],
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        th32ParentProcessID: 0,
        th32ProcessID: 0,
    };

    entry
}

#[cfg(windows)]
pub fn get_process_id_from_name(process_name: &str) -> u32 {
    let mut entry = windows_initialise_process_entry();

    let mut windows_process = WindowsProcesses {
        entry: &mut entry,
        snapshot: unsafe { std::mem::zeroed() },
    };
    let pid = windows_process.get_pid_from_process_name(process_name);

    if pid != 0 {
        println!("The process ID returned: {:?}", pid);
    } else {
        println!("Failed to get valid process ID");
    }

    pid
}