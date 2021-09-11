use crate::{
    audio_endpoint_volume::AudioEndpointVolume,
    audio_session_manager::AudioSessionManager,
    bindings::Windows::Win32::{
        Media::Audio::CoreAudio::IMMDevice,
        Storage::StructuredStorage::PROPVARIANT,
        System::{
            Com::CLSCTX_ALL,
            SystemServices::{
                DEVPKEY_DeviceInterface_FriendlyName, DEVPKEY_Device_DeviceDesc,
                DEVPKEY_Device_FriendlyName,
            },
        },
    },
    bits::{DeviceState, StorageAccessMode},
    property_store::{PropertyKey, PropertyStore},
    string::WinString,
};
use windows::{Abi, Interface};

/// See also: [`IMMDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdevice)
pub struct Device {
    inner: IMMDevice,
}

impl Device {
    pub(crate) fn new(inner: IMMDevice) -> Self {
        Self { inner }
    }

    pub(crate) unsafe fn activate<T>(&self, params: *mut PROPVARIANT) -> windows::Result<T>
    where
        T: Activate,
    {
        let mut raw = None;
        unsafe {
            self.inner
                .Activate(&T::Raw::IID, CLSCTX_ALL.0, params, raw.set_abi())?;
        }
        Ok(T::from_raw(raw.unwrap()))
    }

    pub fn activate_audio_endpoint_volume(&self) -> windows::Result<AudioEndpointVolume> {
        unsafe { self.activate(std::ptr::null_mut()) }
    }

    pub fn activate_audio_session_manager(&self) -> windows::Result<AudioSessionManager> {
        unsafe { self.activate(std::ptr::null_mut()) }
    }

    /// See also: [`IMMDevice::GetId`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getid)
    pub fn get_id(&self) -> windows::Result<WinString> {
        Ok(unsafe { WinString::from_pwstr(self.inner.GetId()?) })
    }

    /// See also: [`IMMDevice::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getstate)
    pub fn get_state(&self) -> windows::Result<DeviceState> {
        Ok(DeviceState::from_raw(unsafe { self.inner.GetState()? }))
    }

    /// See also: [`IMMDevice::OpenPropertyStore`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-openpropertystore)
    pub fn open_property_store(
        &self,
        storage_access_mode: StorageAccessMode,
    ) -> windows::Result<PropertyStore> {
        unsafe {
            self.inner
                .OpenPropertyStore(storage_access_mode.to_raw() as _)
                .map(PropertyStore::new)
        }
    }
}

pub(crate) trait Activate {
    type Raw: windows::Interface;

    fn from_raw(raw: Self::Raw) -> Self;
}

pub const DEVICE_INTERFACE_FRIENDLY_NAME: PropertyKey =
    PropertyKey::from_raw(DEVPKEY_DeviceInterface_FriendlyName);
pub const DEVICE_DESCRIPTION: PropertyKey = PropertyKey::from_raw(DEVPKEY_Device_DeviceDesc);
pub const DEVICE_FRIENDLY_NAME: PropertyKey = PropertyKey::from_raw(DEVPKEY_Device_FriendlyName);
