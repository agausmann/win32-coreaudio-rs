use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::IMMDevice;

/// See also: [`IMMDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdevice)
pub struct Device {
    inner: IMMDevice,
}

impl Device {
    pub(crate) fn new(inner: IMMDevice) -> Self {
        Self { inner }
    }
}
