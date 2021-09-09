use crate::{
    bindings::Windows::Win32::{
        Media::Audio::CoreAudio::{IMMDeviceEnumerator, IMMNotificationClient, MMDeviceEnumerator},
        System::Com::{CoCreateInstance, CLSCTX_ALL},
    },
    bits::{DataFlow, DataFlowMask, DeviceRole, DeviceStateMask},
    device::Device,
    device_collection::DeviceCollection,
    notification_client::{NotificationClient, NotificationClientWrapper},
    string::WinStr,
};

/// See also: [`IMMDeviceEnumerator`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdeviceenumerator)
pub struct DeviceEnumerator {
    inner: IMMDeviceEnumerator,
}

impl DeviceEnumerator {
    pub fn new() -> windows::Result<Self> {
        // Static entrypoint:
        crate::ensure_thread_init();

        let inner = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
        Ok(Self { inner })
    }

    /// See also: [`IMMDeviceEnumerator::EnumAudioEndpoints`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-enumaudioendpoints)
    pub fn enum_audio_endpoints(
        &self,
        data_flow_mask: DataFlowMask,
        state_mask: DeviceStateMask,
    ) -> windows::Result<DeviceCollection> {
        let inner = unsafe {
            self.inner
                .EnumAudioEndpoints(data_flow_mask.to_raw(), state_mask.bits())?
        };
        Ok(DeviceCollection::new(inner))
    }

    /// See also: [`IMMDeviceEnumerator::GetDefaultAudioEndpoint`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-getdefaultaudioendpoint)
    pub fn get_default_audio_endpoint(
        &self,
        data_flow: DataFlow,
        role: DeviceRole,
    ) -> windows::Result<Device> {
        unsafe {
            self.inner
                .GetDefaultAudioEndpoint(data_flow.to_raw(), role.to_raw())
                .map(Device::new)
        }
    }

    /// See also: [`IMMDeviceEnumerator::GetDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-getdevice)
    pub fn get_device(&self, device_id: WinStr) -> windows::Result<Device> {
        unsafe { self.inner.GetDevice(device_id.as_pwstr()).map(Device::new) }
    }

    /// See also: [`IMMDeviceEnumerator::RegisterEndpointNotificationCallback`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-registerendpointnotificationcallback)
    pub fn register_endpoint_notification<T>(
        &self,
        notification_client: T,
    ) -> windows::Result<NotificationClientHandle>
    where
        T: NotificationClient,
    {
        let wrapper =
            IMMNotificationClient::from(NotificationClientWrapper::new(notification_client));
        //TODO addref
        let result = unsafe { self.inner.RegisterEndpointNotificationCallback(&wrapper) };
        if result.is_err() {
            //TODO release
        }
        result?;

        Ok(NotificationClientHandle { inner: wrapper })
    }

    /// See also: [`IMMDeviceEnumerator::UnregisterEndpointNotificationCallback`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-unregisterendpointnotificationcallback)
    pub fn unregister_endpoint_notification(
        &self,
        handle: &NotificationClientHandle,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .UnregisterEndpointNotificationCallback(&handle.inner)?;
        }
        // TODO release
        Ok(())
    }
}

pub struct NotificationClientHandle {
    inner: IMMNotificationClient,
}
