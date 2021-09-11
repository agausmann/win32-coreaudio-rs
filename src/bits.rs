//! Collection of translated bitflag and enumeration types.

use crate::bindings::Windows::Win32::{
    Media::Audio::CoreAudio::{
        eAll, eCapture, eCommunications, eConsole, eMultimedia, eRender, EDataFlow, ERole,
        DEVICE_STATEMASK_ALL, DEVICE_STATE_ACTIVE, DEVICE_STATE_DISABLED, DEVICE_STATE_NOTPRESENT,
        DEVICE_STATE_UNPLUGGED, ENDPOINT_HARDWARE_SUPPORT_METER, ENDPOINT_HARDWARE_SUPPORT_MUTE,
        ENDPOINT_HARDWARE_SUPPORT_VOLUME,
    },
    Storage::StructuredStorage::{STGM_READ, STGM_READWRITE, STGM_WRITE},
};

macro_rules! map_enum {
    ($(
        $(#[$meta:meta])*
        $vis:vis enum $name:ident: $maptype:ident {
            $($varname:ident = $varmap:expr,)*
        }
    )*) => {$(
        $(#[$meta])*
        $vis enum $name {
            $($varname),*
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) fn from_raw(raw: $maptype) -> Self {
                match raw {
                    $(x if x == $varmap => Self::$varname,)*
                    _ => panic!("invalid raw value {:?}", raw)
                }
            }

            #[allow(dead_code)]
            pub(crate) fn to_raw(&self) -> $maptype {
                match self {
                    $(Self::$varname => $varmap),*
                }
            }
        }
    )*}
}

map_enum! {
    /// See also: [`EDataFlow`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-edataflow)
    pub enum DataFlow: EDataFlow {
        Render = eRender,
        Capture = eCapture,
    }

    /// See also: [`EDataFlow`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-edataflow)
    pub enum DataFlowMask: EDataFlow {
        Render = eRender,
        Capture = eCapture,
        All = eAll,
    }

    /// See also: [`DEVICE_STATE_XXXX Constants`](https://docs.microsoft.com/en-us/windows/win32/coreaudio/device-state-xxx-constants)
    pub enum DeviceState: u32 {
        Active = DEVICE_STATE_ACTIVE,
        Disabled = DEVICE_STATE_DISABLED,
        NotPresent = DEVICE_STATE_NOTPRESENT,
        Unplugged = DEVICE_STATE_UNPLUGGED,
    }

    /// See also: [`ERole`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-erole)
    pub enum DeviceRole: ERole {
        Console = eConsole,
        Multimedia = eMultimedia,
        Communications = eCommunications,
    }

    pub enum StorageAccessMode: i32 {
        Read = STGM_READ,
        Write = STGM_WRITE,
        ReadWrite = STGM_READWRITE,
    }
}

bitflags::bitflags! {
    /// See also: [`DEVICE_STATE_XXXX Constants`](https://docs.microsoft.com/en-us/windows/win32/coreaudio/device-state-xxx-constants)
    pub struct DeviceStateMask: u32 {
        const ACTIVE = DEVICE_STATE_ACTIVE;
        const DISABLED = DEVICE_STATE_DISABLED;
        const NOT_PRESENT = DEVICE_STATE_NOTPRESENT;
        const UNPLUGGED = DEVICE_STATE_UNPLUGGED;
        const ALL = DEVICE_STATEMASK_ALL;
    }

    /// See also: [`ENDPOINT_HARDWARE_SUPPORT_XXX Constants`](https://docs.microsoft.com/en-us/windows/win32/coreaudio/endpoint-hardware-support-xxx-constants)
    pub struct HardwareSupportMask: u32 {
        const MUTE = ENDPOINT_HARDWARE_SUPPORT_MUTE;
        const METER = ENDPOINT_HARDWARE_SUPPORT_METER;
        const VOLUME = ENDPOINT_HARDWARE_SUPPORT_VOLUME;
    }
}
