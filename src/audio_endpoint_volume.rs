use windows::Guid;

use crate::{
    audio_endpoint_volume_callback::{
        AudioEndpointVolumeCallback, AudioEndpointVolumeCallbackWrapper,
    },
    bindings::Windows::Win32::Media::Audio::CoreAudio::{
        IAudioEndpointVolume, IAudioEndpointVolumeCallback,
    },
    bits::HardwareSupportMask,
    device::Activate,
};

/// See also: [`IAudioEndpointVolume`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume)
pub struct AudioEndpointVolume {
    inner: IAudioEndpointVolume,
}

impl Activate for AudioEndpointVolume {
    type Raw = IAudioEndpointVolume;

    fn from_raw(inner: Self::Raw) -> Self {
        Self { inner }
    }
}

impl AudioEndpointVolume {
    /// See also: [`IAudioEndpointVolume::GetChannelCount`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getchannelcount)
    pub fn get_channel_count(&self) -> u32 {
        unsafe { self.inner.GetChannelCount().unwrap() }
    }

    /// See also: [`IAudioEndpointVolume::GetChannelVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getchannelvolumelevel)
    pub fn get_channel_volume_level(&self, channel: u32) -> Option<f32> {
        unsafe { self.inner.GetChannelVolumeLevel(channel).ok() }
    }

    /// See also: [`IAudioEndpointVolume::GetChannelVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getchannelvolumelevelscalar)
    pub fn get_channel_volume_level_scalar(&self, channel: u32) -> Option<f32> {
        unsafe { self.inner.GetChannelVolumeLevelScalar(channel).ok() }
    }

    /// See also: [`IAudioEndpointVolume::GetMasterVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmastervolumelevel)
    pub fn get_master_volume_level(&self) -> f32 {
        unsafe { self.inner.GetMasterVolumeLevel().unwrap() }
    }

    /// See also: [`IAudioEndpointVolume::GetMasterVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmastervolumelevelscalar)
    pub fn get_master_volume_level_scalar(&self) -> f32 {
        unsafe { self.inner.GetMasterVolumeLevelScalar().unwrap() }
    }

    /// See also: [`IAudioEndpointVolume::GetMute`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmute)
    pub fn get_mute(&self) -> bool {
        unsafe { self.inner.GetMute().unwrap().into() }
    }

    /// See also: [`IAudioEndpointVolume::GetVolumeRange`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getvolumerange)
    pub fn get_volume_range(&self) -> VolumeRange {
        let mut volume_range = VolumeRange {
            min_db: 0.0,
            max_db: 0.0,
            increment_db: 0.0,
        };
        unsafe {
            self.inner
                .GetVolumeRange(
                    &mut volume_range.min_db,
                    &mut volume_range.max_db,
                    &mut volume_range.increment_db,
                )
                .unwrap()
        };
        volume_range
    }

    /// See also: [`IAudioEndpointVolume::GetVolumeStepInfo`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getvolumestepinfo)
    pub fn get_volume_step_info(&self) -> VolumeStepInfo {
        let mut volume_step_info = VolumeStepInfo {
            current_step: 0,
            num_steps: 0,
        };
        unsafe {
            self.inner
                .GetVolumeStepInfo(
                    &mut volume_step_info.current_step,
                    &mut volume_step_info.num_steps,
                )
                .unwrap()
        };
        volume_step_info
    }

    /// See also: [`IAudioEndpointVolume::QueryHardwareSupport`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-queryhardwaresupport)
    pub fn query_hardware_support(&self) -> HardwareSupportMask {
        let raw = unsafe { self.inner.QueryHardwareSupport().unwrap() };
        HardwareSupportMask::from_bits(raw).expect("invalid mask")
    }

    /// See also: [`IAudioEndpointVolume::RegisterControlChangeNotify`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-registercontrolchangenotify)
    pub fn register_control_change_notify<T>(
        &self,
        callback: T,
    ) -> AudioEndpointVolumeCallbackHandle
    where
        T: AudioEndpointVolumeCallback,
    {
        let callback =
            IAudioEndpointVolumeCallback::from(AudioEndpointVolumeCallbackWrapper::new(callback));
        unsafe { self.inner.RegisterControlChangeNotify(&callback).unwrap() };
        AudioEndpointVolumeCallbackHandle { inner: callback }
    }

    /// See also: [`IAudioEndpointVolume::UnregisterControlChangeNotify`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-unregistercontrolchangenotify)
    pub fn unregister_control_change_notify(
        &self,
        handle: &AudioEndpointVolumeCallbackHandle,
    ) -> windows::Result<()> {
        unsafe { self.inner.UnregisterControlChangeNotify(&handle.inner) }
    }

    /// See also: [`IAudioEndpointVolume::SetChannelVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-setchannelvolumelevel)
    pub fn set_channel_volume_level(
        &self,
        channel: u32,
        level_db: f32,
        event_context: Option<&Guid>,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .SetChannelVolumeLevel(channel, level_db, as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioEndpointVolume::SetChannelVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-setchannelvolumelevelscalar)
    pub fn set_channel_volume_level_scalar(
        &self,
        channel: u32,
        level: f32,
        event_context: Option<&Guid>,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .SetChannelVolumeLevelScalar(channel, level, as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioEndpointVolume::SetMasterVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-setmastervolumelevel)
    pub fn set_master_volume_level(
        &self,
        level_db: f32,
        event_context: Option<&Guid>,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .SetMasterVolumeLevel(level_db, as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioEndpointVolume::SetMasterVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-setmastervolumelevelscalar)
    pub fn set_master_volume_level_scalar(
        &self,
        level: f32,
        event_context: Option<&Guid>,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .SetMasterVolumeLevelScalar(level, as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioEndpointVolume::SetMute`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-setmute)
    pub fn set_mute(&self, mute: bool, event_context: Option<&Guid>) -> windows::Result<()> {
        unsafe {
            self.inner.SetMute(
                mute,
                event_context
                    .map(|x| x as *const _)
                    .unwrap_or(std::ptr::null()),
            )
        }
    }

    /// See also: [`IAudioEndpointVolume::VolumeStepDown`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-volumestepdown)
    pub fn volume_step_down(&self, event_context: Option<&Guid>) -> windows::Result<()> {
        unsafe { self.inner.VolumeStepDown(as_raw_or_null(event_context)) }
    }

    /// See also: [`IAudioEndpointVolume::VolumeStepUp`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-volumestepup)
    pub fn volume_step_up(&self, event_context: Option<&Guid>) -> windows::Result<()> {
        unsafe { self.inner.VolumeStepUp(as_raw_or_null(event_context)) }
    }
}

#[non_exhaustive]
pub struct VolumeRange {
    pub min_db: f32,
    pub max_db: f32,
    pub increment_db: f32,
}

#[non_exhaustive]
pub struct VolumeStepInfo {
    pub current_step: u32,
    pub num_steps: u32,
}

pub struct AudioEndpointVolumeCallbackHandle {
    inner: IAudioEndpointVolumeCallback,
}

fn as_raw_or_null<T>(option: Option<&T>) -> *const T {
    option.map(|x| x as *const _).unwrap_or(std::ptr::null())
}
