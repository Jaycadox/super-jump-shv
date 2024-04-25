use std::ffi::c_void;

use scripthookv_lite::{
    natives::{misc, player},
    script_register, script_unregister, script_wait,
};
use windows::Win32::{Foundation::HMODULE, System::SystemServices};

extern "C" fn script_main() {
    loop {
        unsafe {
            misc::set_super_jump_this_frame(player::player_id());
        }
        unsafe { script_wait(0) };
    }
}

#[no_mangle]
pub extern "system" fn DllMain(instance: HMODULE, reason: u32, _: *mut c_void) -> i32 {
    match reason {
        SystemServices::DLL_PROCESS_ATTACH => {
            unsafe { script_register(instance, script_main) };
        }
        SystemServices::DLL_PROCESS_DETACH => {
            unsafe { script_unregister(instance) };
        }
        _ => {}
    }
    1
}
