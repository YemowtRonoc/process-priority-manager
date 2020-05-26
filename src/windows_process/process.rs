#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
use winapi::um::tlhelp32::PROCESSENTRY32;
#[cfg(windows)]
use winapi::um::winnt::HANDLE;

#[cfg(windows)]
struct WindowsProcessPriority {
    snapshot: HANDLE,
    entry: PROCESSENTRY32,
}

#[cfg(windows)]
impl WindowsProcessPriority {
    #[cfg(windows)]
    fn set_process_priority(&self, process_handle: HANDLE, priority_level: u32) -> bool {
        use winapi::shared::minwindef::{BOOL, FALSE};
        use winapi::um::processthreadsapi::SetPriorityClass;

        let result: BOOL = unsafe { SetPriorityClass(process_handle, priority_level) };

        if result == FALSE {
            println!(
                "Failed to set {:?} priority on process: {:?}",
                priority_level, self.entry.th32ProcessID
            );
            false
        } else {
            true
        }
    }

    #[cfg(windows)]
    fn set_priority_for_process_(&self, process_priority: u32) -> u32 {
        use winapi::shared::minwindef::FALSE;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::{GetProcessId, OpenProcess};
        use winapi::um::winnt::PROCESS_ALL_ACCESS;

        let process_handle =
            unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, self.entry.th32ProcessID) };

        // Do stuff..
        let pid = unsafe { GetProcessId(process_handle) };

        self.set_process_priority(process_handle, process_priority);

        unsafe {
            CloseHandle(process_handle);
        };

        pid as u32
    }

    #[cfg(windows)]
    fn windows_process_32_first(&mut self) -> bool {
        use winapi::shared::minwindef::TRUE;
        use winapi::um::tlhelp32::Process32First;

        let result = unsafe { Process32First(self.snapshot, &mut self.entry) };

        result == TRUE
    }

    #[cfg(windows)]
    fn windows_process_32_next(&mut self) -> bool {
        use winapi::shared::minwindef::TRUE;
        use winapi::um::tlhelp32::Process32Next;

        let result = unsafe { Process32Next(self.snapshot, &mut self.entry) };

        result == TRUE
    }

    #[cfg(windows)]
    fn windows_create_toolhelp_32_snapshot(&self) -> HANDLE {
        use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS};

        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
    }

    #[cfg(windows)]
    fn windows_close_handle(&self) -> i32 {
        use winapi::um::handleapi::CloseHandle;

        unsafe { CloseHandle(self.snapshot) }
    }

    #[cfg(windows)]
    fn windows_initialise_process_entry(&self) -> PROCESSENTRY32 {
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
    pub fn set_priority_for_process_if_running(
        &mut self,
        process_name: &str,
        process_priority: u32,
    ) -> u32 {
        use std::ffi::CStr;
        self.snapshot = self.windows_create_toolhelp_32_snapshot();
        self.entry = self.windows_initialise_process_entry();
        let mut pid = 0;

        if self.windows_process_32_first() {
            while self.windows_process_32_next() {
                let current_process_name = unsafe { CStr::from_ptr(self.entry.szExeFile.as_ptr()) };
                if current_process_name.to_str().unwrap() == process_name {
                    pid = self.set_priority_for_process_(process_priority);
                    break;
                }
            }
        }
        self.windows_close_handle();

        pid as u32
    }
}

#[cfg(windows)]
pub fn set_cpu_priority_for_process(process_name: &str, process_priority: u32) -> u32 {
    let mut windows_process = WindowsProcessPriority {
        entry: unsafe { std::mem::zeroed() },
        snapshot: unsafe { std::mem::zeroed() },
    };
    let pid = windows_process.set_priority_for_process_if_running(process_name, process_priority);

    if pid == 0 {
        println!("Failed to get valid process ID");
    }

    pid
}

#[cfg(not(windows))]
pub fn set_cpu_priority_for_process(_process_name: &str, _process_priority: u32) -> u32 {
    println!("This function has not yet been implemented on this platform...");
    0
}

pub fn enumerate_priority_level(priority_str: &str) -> u32 {
    use winapi::um::winbase::{
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
        IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, PROCESS_MODE_BACKGROUND_BEGIN,
        PROCESS_MODE_BACKGROUND_END, REALTIME_PRIORITY_CLASS,
    };

    const REALTIME: &str = "realtime";
    const HIGH: &str = "high";
    const ABOVE_NORMAL: &str = "above normal";
    const NORMAL: &str = "normal";
    const BELOW_NORMAL: &str = "below normal";
    const IDLE: &str = "idle";
    const BACKGROUND_BEGIN: &str = "background begin";
    const BACKGROUND_END: &str = "background end";

    match priority_str.to_lowercase().as_str() {
        REALTIME => REALTIME_PRIORITY_CLASS,
        HIGH => HIGH_PRIORITY_CLASS,
        ABOVE_NORMAL => ABOVE_NORMAL_PRIORITY_CLASS,
        NORMAL => NORMAL_PRIORITY_CLASS,
        BELOW_NORMAL => BELOW_NORMAL_PRIORITY_CLASS,
        IDLE => IDLE_PRIORITY_CLASS,
        BACKGROUND_BEGIN => PROCESS_MODE_BACKGROUND_BEGIN,
        BACKGROUND_END => PROCESS_MODE_BACKGROUND_END,
        _ => 0,
    }
}
