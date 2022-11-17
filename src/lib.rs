use sdk::{log_info, get_data_as_str};
use serde_json::Value;

mod sdk;
mod types;

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size as _);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

#[no_mangle]
// This handler will be matched by the default Project event strategy in W3bstream
pub extern "C" fn start(event_id: i32) -> i32 {
    log_info("start from rust");

    let data_u8 = match sdk::get_data(event_id) {
        Some(data) => data,
        _ => {
            sdk::log_error("failed to get data from event");
            return -1;
        }
    };
    
    let data_str = match String::from_utf8(data_u8) {
        Ok(data) => data,
        _ => {
            sdk::log_error("failed to convert data to string");
            return -1;
        }
    };

    sdk::log_info(&format!("data: {}", data_str));
    sdk::log_info(&format!("data: {}", data_str));
    return 0;
}