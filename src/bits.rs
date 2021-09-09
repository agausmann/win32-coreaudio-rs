//! Collection of translated bitflag and enumeration types.

use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::{
    eAll, eCapture, eCommunications, eConsole, eMultimedia, eRender, EDataFlow, ERole,
    DEVICE_STATEMASK_ALL, DEVICE_STATE_ACTIVE, DEVICE_STATE_DISABLED, DEVICE_STATE_NOTPRESENT,
    DEVICE_STATE_UNPLUGGED,
};

/// See also: [`EDataFlow`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-edataflow)
pub enum DataFlow {
    Render,
    Capture,
}

impl DataFlow {
    pub(crate) fn from_raw(raw: EDataFlow) -> Self {
        if raw == eRender {
            Self::Render
        } else if raw == eCapture {
            Self::Capture
        } else {
            panic!("invalid data flow {:?}", raw);
        }
    }

    pub(crate) fn to_raw(&self) -> EDataFlow {
        match self {
            Self::Render => eRender,
            Self::Capture => eCapture,
        }
    }
}

/// See also: [`EDataFlow`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-edataflow)
pub enum DataFlowMask {
    Render,
    Capture,
    All,
}

impl DataFlowMask {
    pub(crate) fn from_raw(raw: EDataFlow) -> Self {
        if raw == eRender {
            Self::Render
        } else if raw == eCapture {
            Self::Capture
        } else if raw == eAll {
            Self::All
        } else {
            panic!("invalid data flow mask {:?}", raw);
        }
    }

    pub(crate) fn to_raw(&self) -> EDataFlow {
        match self {
            Self::Render => eRender,
            Self::Capture => eCapture,
            Self::All => eAll,
        }
    }
}

/// See also: [`DEVICE_STATE_XXXX Constants`](https://docs.microsoft.com/en-us/windows/win32/coreaudio/device-state-xxx-constants)
pub enum DeviceState {
    Active,
    Disabled,
    NotPresent,
    Unplugged,
}

impl DeviceState {
    pub(crate) fn from_raw(raw: u32) -> Self {
        if raw == DEVICE_STATE_ACTIVE {
            Self::Active
        } else if raw == DEVICE_STATE_DISABLED {
            Self::Disabled
        } else if raw == DEVICE_STATE_NOTPRESENT {
            Self::NotPresent
        } else if raw == DEVICE_STATE_UNPLUGGED {
            Self::Unplugged
        } else {
            panic!("invalid device state {:?}", raw);
        }
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
}

/// See also: [`ERole`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-erole)
pub enum DeviceRole {
    Console,
    Multimedia,
    Communications,
}

impl DeviceRole {
    pub(crate) fn from_raw(raw: ERole) -> Self {
        if raw == eConsole {
            Self::Console
        } else if raw == eMultimedia {
            Self::Multimedia
        } else if raw == eCommunications {
            Self::Communications
        } else {
            panic!("invalid device role {:?}", raw);
        }
    }

    pub(crate) fn to_raw(&self) -> ERole {
        match self {
            Self::Console => eConsole,
            Self::Multimedia => eMultimedia,
            Self::Communications => eCommunications,
        }
    }
}
