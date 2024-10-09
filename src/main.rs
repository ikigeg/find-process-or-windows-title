use std::env;
use std::ptr::null_mut;
use widestring::U16CString;
use winapi::shared::minwindef::{BOOL, LPARAM, UINT};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleBaseNameW;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use std::sync::atomic::{AtomicBool, Ordering};

// Struct to hold filters
struct Filters {
    window_title: Option<String>,
    process_name: Option<String>,
}

static mut FILTERS: Filters = Filters { window_title: None, process_name: None };
static MATCH_FOUND: AtomicBool = AtomicBool::new(false);

// Function to get the parent process ID (PPID)
unsafe fn get_parent_process_id(pid: u32) -> u32 {
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if snapshot == INVALID_HANDLE_VALUE {
        return 0; // If we can't create a snapshot, return 0
    }

    let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
    process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    if Process32First(snapshot, &mut process_entry) != 0 {
        loop {
            if process_entry.th32ProcessID == pid {
                let parent_pid = process_entry.th32ParentProcessID;
                CloseHandle(snapshot);
                return parent_pid;
            }

            if Process32Next(snapshot, &mut process_entry) == 0 {
                break;
            }
        }
    }

    CloseHandle(snapshot);
    0
}

// Callback function to be called for each enumerated window
extern "system" fn enum_windows_proc(h_wnd: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let length = GetWindowTextLengthW(h_wnd);
        if length == 0 {
            return 1;
        }

        let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
        GetWindowTextW(h_wnd, buffer.as_mut_ptr(), length + 1);

        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(h_wnd, &mut process_id);

        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if process_handle.is_null() {
            return 1;
        }

        let mut process_name_buffer: [u16; 260] = [0; 260];
        GetModuleBaseNameW(process_handle, null_mut(), process_name_buffer.as_mut_ptr(), process_name_buffer.len() as UINT);

        let process_name = U16CString::from_vec_unchecked(process_name_buffer.to_vec()).to_string_lossy();
        let window_title = U16CString::from_vec_unchecked(buffer).to_string_lossy();

        // Get Parent Process ID
        let parent_process_id = get_parent_process_id(process_id);

        let title_matches = FILTERS.window_title.as_ref().map_or(true, |f| window_title.to_lowercase().contains(f));
        let name_matches = FILTERS.process_name.as_ref().map_or(true, |f| process_name.to_lowercase().contains(f));

        // Print match information
        if title_matches && name_matches {
            println!("Title: {}, Process: {}, PID: {}, PPID: {}", window_title, process_name, process_id, parent_process_id);
            MATCH_FOUND.store(true, Ordering::SeqCst); // Set the match found flag
            return 1; // Found a match
        }

        CloseHandle(process_handle);
    }

    1 // Continue enumerating windows
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Collect filters from command line arguments
    for i in 1..args.len() {
        if let Some(value) = args.get(i + 1) {
            match args[i].as_str() {
                "-wt" => unsafe { FILTERS.window_title = Some(value.to_lowercase()); },
                "-pn" => unsafe { FILTERS.process_name = Some(value.to_lowercase()); },
                _ => {}
            }
        }
    }

    unsafe { EnumWindows(Some(enum_windows_proc), 0); }

    // Check if no matches were found
    unsafe {
        if (FILTERS.window_title.is_some() || FILTERS.process_name.is_some()) && !MATCH_FOUND.load(Ordering::SeqCst) {
            println!("No matches!");
        }
    }
}
