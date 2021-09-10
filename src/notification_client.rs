use crate::bindings::*;
use crate::bits::{DataFlow, DeviceRole, DeviceState};
use crate::string::WinStr;
use crate::{
    bindings::Windows::Win32::{
        Foundation::PWSTR,
        Media::Audio::CoreAudio::{EDataFlow, ERole},
        System::PropertiesSystem::PROPERTYKEY,
    },
    property_store::PropertyKey,
};

/// See also: [`IMMNotificationClient`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immnotificationclient)
pub trait NotificationClient: 'static {
    /// See also: [`IMMNotificationClient::OnDefaultDeviceChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondefaultdevicechanged)
    fn on_default_device_changed(
        &mut self,
        data_flow: DataFlow,
        role: DeviceRole,
        device_id: WinStr,
    ) -> windows::Result<()> {
        let _ = (data_flow, role, device_id);
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceAdded`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondeviceadded)
    fn on_device_added(&mut self, device_id: WinStr) -> windows::Result<()> {
        let _ = device_id;
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceRemoved`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondeviceremoved)
    fn on_device_removed(&mut self, device_id: WinStr) -> windows::Result<()> {
        let _ = device_id;
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceStateChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondevicestatechanged)
    fn on_device_state_changed(
        &mut self,
        device_id: WinStr,
        state: DeviceState,
    ) -> windows::Result<()> {
        let _ = (device_id, state);
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnPropertyValueChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-onpropertyvaluechanged)
    fn on_property_value_changed(
        &mut self,
        device_id: WinStr,
        property_key: PropertyKey,
    ) -> windows::Result<()> {
        let _ = (device_id, property_key);
        Ok(())
    }
}

#[windows::implement(Windows::Win32::Media::Audio::CoreAudio::IMMNotificationClient)]
pub(crate) struct NotificationClientWrapper {
    inner: Box<dyn NotificationClient>,
}

impl NotificationClientWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: NotificationClient,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// Impl IMMNotificationClient
#[allow(non_snake_case)]
impl NotificationClientWrapper {
    fn OnDefaultDeviceChanged(
        &mut self,
        flow: EDataFlow,
        role: ERole,
        device_id: PWSTR,
    ) -> windows::Result<()> {
        self.inner.on_default_device_changed(
            DataFlow::from_raw(flow),
            DeviceRole::from_raw(role),
            unsafe { WinStr::from_pwstr(&device_id) },
        )
    }

    fn OnDeviceAdded(&mut self, device_id: PWSTR) -> windows::Result<()> {
        self.inner
            .on_device_added(unsafe { WinStr::from_pwstr(&device_id) })
    }

    fn OnDeviceRemoved(&mut self, device_id: PWSTR) -> windows::Result<()> {
        self.inner
            .on_device_removed(unsafe { WinStr::from_pwstr(&device_id) })
    }

    fn OnDeviceStateChanged(&mut self, device_id: PWSTR, new_state: u32) -> windows::Result<()> {
        self.inner.on_device_state_changed(
            unsafe { WinStr::from_pwstr(&device_id) },
            DeviceState::from_raw(new_state),
        )
    }

    fn OnPropertyValueChanged(
        &mut self,
        device_id: PWSTR,
        key: PROPERTYKEY,
    ) -> windows::Result<()> {
        self.inner.on_property_value_changed(
            unsafe { WinStr::from_pwstr(&device_id) },
            PropertyKey::from_raw(key),
        )
    }
}
