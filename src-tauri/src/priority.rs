use windows::Win32::Foundation::{CloseHandle};
use windows::Win32::System::Threading::{OpenProcess, SetPriorityClass, PROCESS_CREATION_FLAGS, PROCESS_SET_INFORMATION,};

pub fn set_priority(pid: u32, class: PROCESS_CREATION_FLAGS) -> windows::core::Result<()> {
    unsafe {
        let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;
        let result = SetPriorityClass(handle, class);
        CloseHandle(handle)?;
        result
    }
}