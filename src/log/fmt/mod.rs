pub mod facility;
pub mod formatter;
pub mod index;
pub mod layer;
pub mod log_layer;
pub mod log_value;
pub mod severity;
pub mod storage;
pub mod util;
pub mod value;

use alloc::string::String;

pub fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}
