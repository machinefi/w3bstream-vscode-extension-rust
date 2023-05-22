use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn start(_: i32) -> i32 {
    log_info("hello world!");
    return 0;
}