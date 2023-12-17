use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr};

use winapi::{
    shared::{
        minwindef::{DWORD, FALSE},
        windef::HWND,
    },
    um::{
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::GetModuleFileNameExW,
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, WCHAR},
        winuser::{
            GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
        },
    },
};

fn get_window_text(hwnd: HWND) -> Option<OsString> {
    // Get the length of the window title
    let length = unsafe { GetWindowTextLengthW(hwnd) };
    if length == 0 {
        return None; // No window title
    }

    // Allocate a buffer to store the window title
    let mut buffer: Vec<u16> = Vec::with_capacity((length + 1) as usize);

    // Get the window title
    let result = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), (length + 1) as i32) };
    if result == 0 {
        return None; // Failed to get window title
    }

    // Set the actual length of the buffer based on the result
    unsafe {
        buffer.set_len(result as usize);
    }

    // Convert the buffer to a Rust String
    let title = OsString::from_wide(&buffer);
    Some(title)
}

pub fn get_foreground_window_title() -> Option<OsString> {
    let hwnd = unsafe { GetForegroundWindow() };
    get_window_text(hwnd)
}

pub fn get_foreground_window_exe_name() -> Result<String, &'static str> {
    unsafe {
        let hwnd = GetForegroundWindow();

        // Get the process ID of the foreground window
        let mut process_id: DWORD = 0;

        GetWindowThreadProcessId(hwnd, &mut process_id);

        // Open the process
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            FALSE,
            process_id,
        );

        if process_handle.is_null() {
            return Err("Failed to open process");
        }

        // Get the executable path
        let mut buffer: [WCHAR; 260] = [0; 260];
        let size = GetModuleFileNameExW(
            process_handle,
            ptr::null_mut(),
            buffer.as_mut_ptr(),
            buffer.len() as DWORD,
        );

        // Close the process handle
        CloseHandle(process_handle);

        if size == 0 {
            return Err("Failed to get module file name");
        }

        // Convert the WCHAR buffer to a Rust String
        let exe_name = OsString::from_wide(&buffer[..size as usize]);
        Ok(exe_name.to_string_lossy().into_owned())
    }
}
