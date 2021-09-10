use crate::{
    bindings::Windows::Win32::{
        Media::Audio::CoreAudio::IMMDevice,
        System::SystemServices::{
            DEVPKEY_DeviceInterface_FriendlyName, DEVPKEY_Device_DeviceDesc,
            DEVPKEY_Device_FriendlyName,
        },
    },
    bits::{DeviceState, StorageAccessMode},
    property_store::{PropertyKey, PropertyStore},
    string::WinString,
};

/// See also: [`IMMDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdevice)
pub struct Device {
    inner: IMMDevice,
}

impl Device {
    pub(crate) fn new(inner: IMMDevice) -> Self {
        Self { inner }
    }

    //TODO IMMDevice::Activate

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

pub const DEVICE_INTERFACE_FRIENDLY_NAME: PropertyKey =
    PropertyKey::from_raw(DEVPKEY_DeviceInterface_FriendlyName);
pub const DEVICE_DESCRIPTION: PropertyKey = PropertyKey::from_raw(DEVPKEY_Device_DeviceDesc);
pub const DEVICE_FRIENDLY_NAME: PropertyKey = PropertyKey::from_raw(DEVPKEY_Device_FriendlyName);
