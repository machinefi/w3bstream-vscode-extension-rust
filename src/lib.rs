use sdk::{log_info, get_data_as_str};

mod sdk;
mod types;

#[no_mangle]
// This handler will be matched by the default Project event strategy in W3bstream
pub extern "C" fn start(event_id: i32) -> i32 {
    log_info(
        &format!("Start handler called with event_id: {}", event_id));

    let payload = get_data_as_str(event_id).unwrap();
    log_info(&format!("event data as string: {}", payload));
    return 0;
}