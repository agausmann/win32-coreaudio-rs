#![warn(unsafe_op_in_unsafe_fn)]

mod bindings {
    #![allow(unsafe_op_in_unsafe_fn)]

    windows::include_bindings!();
}

pub mod audio_endpoint_volume;
pub mod audio_endpoint_volume_callback;
pub mod bits;
pub mod device;
pub mod device_collection;
pub mod device_enumerator;
pub mod notification_client;
pub mod property_store;
pub mod string;

use crate::bindings::Windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use std::sync::Once;

/// Make sure this is called at every static entrypoint to this crate.
pub(crate) fn ensure_thread_init() {
    thread_local! {
        static INIT_ONCE: Once = Once::new();
    }
    INIT_ONCE.with(|init_once| {
        init_once.call_once(|| unsafe {
            CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED).unwrap()
        })
    })
}
