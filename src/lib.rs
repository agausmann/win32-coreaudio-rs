#![warn(unsafe_op_in_unsafe_fn)]

mod bindings {
    #![allow(unsafe_op_in_unsafe_fn)]

    windows::include_bindings!();
}

mod audio_endpoint_volume;
mod audio_endpoint_volume_callback;
mod audio_session_control;
mod audio_session_enumerator;
mod audio_session_events;
mod audio_session_manager;
mod audio_session_notification;
mod audio_volume_duck_notification;
mod bits;
mod device;
mod device_collection;
mod device_enumerator;
mod notification_client;
mod property_store;
mod simple_audio_volume;
pub mod string;
pub(crate) mod util;

pub use self::{
    audio_endpoint_volume::{AudioEndpointVolume, AudioEndpointVolumeCallbackHandle},
    audio_endpoint_volume_callback::{AudioEndpointVolumeCallback, NotificationData},
    audio_session_control::{AudioSessionControl, AudioSessionControl2, AudioSessionEventsHandle},
    audio_session_enumerator::{AudioSessionEnumerator, AudioSessionIter},
    audio_session_events::AudioSessionEvents,
    audio_session_manager::{
        AudioSessionManager, AudioSessionManager2, AudioSessionNotificationHandle,
        AudioVolumeDuckNotificationHandle,
    },
    audio_session_notification::AudioSessionNotification,
    audio_volume_duck_notification::AudioVolumeDuckNotification,
    bits::{
        AudioSessionDisconnectReason, AudioSessionState, DataFlow, DataFlowMask, DeviceRole,
        DeviceState, DeviceStateMask, HardwareSupportMask, StorageAccessMode,
    },
    device::{Device, DEVICE_DESCRIPTION, DEVICE_FRIENDLY_NAME, DEVICE_INTERFACE_FRIENDLY_NAME},
    device_collection::{DeviceCollection, DeviceIter},
    device_enumerator::{DeviceEnumerator, NotificationClientHandle},
    notification_client::NotificationClient,
    property_store::{Property, PropertyKey, PropertyStore},
    simple_audio_volume::SimpleAudioVolume,
};

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
