use windows::Win32::Foundation::{CloseHandle};
use windows::Win32::System::Threading::{OpenProcess, SetPriorityClass, PROCESS_CREATION_FLAGS, PROCESS_SET_INFORMATION,};
use windows::Win32::System::Threading::{ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,};

pub fn set_priority(pid: u32, class: PROCESS_CREATION_FLAGS) -> windows::core::Result<()> {
    unsafe {
        let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;
        let result = SetPriorityClass(handle, class);
        CloseHandle(handle)?;
        result
    }
}

pub fn class_from_str(s: &str) -> Option<PROCESS_CREATION_FLAGS> {
    Some(match s {
        "Idle" => IDLE_PRIORITY_CLASS,
        "BelowNormal" => BELOW_NORMAL_PRIORITY_CLASS,
        "Normal" => NORMAL_PRIORITY_CLASS,
        "AboveNormal" => ABOVE_NORMAL_PRIORITY_CLASS,
        "High" => HIGH_PRIORITY_CLASS,
        _ => return None,
    })
}