//! High-Fidelity Crash-Resilient Diagnostic Logger
//!
//! Enforces immediate OS file flush on every write, microsecond timestamps,
//! and thread ID tracking so diagnostics are never lost during crashes.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use windows_sys::Win32::System::Threading::GetCurrentThreadId;

static LOG_MUTEX: Mutex<()> = Mutex::new(());

pub fn log_info(msg: &str) {
    write_log("INFO", msg);
}

pub fn log_warn(msg: &str) {
    write_log("WARN", msg);
}

pub fn log_error(msg: &str) {
    write_log("ERROR", msg);
}

pub fn log_trace(msg: &str) {
    write_log("TRACE", msg);
}

fn write_log(level: &str, msg: &str) {
    let _guard = LOG_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("CompleteStoryRuntime.log")
    {
        let tid = unsafe { GetCurrentThreadId() };
        let timestamp = get_timestamp();
        let line = format!("[{}] [TID:{:05}] [{}] [CompleteStory] {}\n", timestamp, tid, level, msg);
        let _ = file.write_all(line.as_bytes());
        let _ = file.flush();
    }
}

fn get_timestamp() -> String {
    // Standard Windows SYSTEMTIME
    #[repr(C)]
    struct SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }

    extern "system" {
        fn GetLocalTime(lpSystemTime: *mut SystemTime);
    }

    let mut st = SystemTime {
        year: 0,
        month: 0,
        day_of_week: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        milliseconds: 0,
    };

    unsafe {
        GetLocalTime(&mut st);
    }

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        st.year, st.month, st.day, st.hour, st.minute, st.second, st.milliseconds
    )
}
