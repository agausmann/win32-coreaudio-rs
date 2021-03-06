fn main() {
    // Does not depend on any other files.
    println!("cargo:rerun-if-changed=build.rs");

    windows::build! {
        Windows::Win32::{
            Globalization::lstrlenW,
            Media::Audio::CoreAudio::{
                DEVICE_STATE_ACTIVE, DEVICE_STATE_DISABLED, DEVICE_STATE_NOTPRESENT,
                DEVICE_STATE_UNPLUGGED, DEVICE_STATEMASK_ALL, IMMDeviceEnumerator,
                IMMNotificationClient, MMDeviceEnumerator, IMMDevice, IAudioSessionManager2,
                IAudioEndpointVolume, IAudioEndpointVolumeCallback, IMMDeviceCollection,
                IAudioSessionNotification, IAudioSessionControl, IAudioSessionEvents,
                AudioSessionState, AudioSessionDisconnectReason, IAudioSessionControl2,
                IAudioSessionEnumerator, ISimpleAudioVolume, ENDPOINT_HARDWARE_SUPPORT_METER,
                ENDPOINT_HARDWARE_SUPPORT_MUTE, ENDPOINT_HARDWARE_SUPPORT_VOLUME,
                IAudioVolumeDuckNotification,
            },
            Storage::StructuredStorage::{STGM_READ, STGM_READWRITE, STGM_WRITE},
            System::{
                Com::{CoInitializeEx, CoCreateInstance, CoTaskMemFree, CLSCTX},
                OleAutomation::VARENUM,
                PropertiesSystem::{IPropertyStore, PropVariantToStringAlloc},
                SystemServices::{
                    DEVPKEY_DeviceInterface_FriendlyName, DEVPKEY_Device_DeviceDesc,
                    DEVPKEY_Device_FriendlyName,
                },
                Memory::LocalFree,
            },
            UI::Shell::{StrDupW, StrCmpW},
        },
    }
}
