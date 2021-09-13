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
    util::as_raw_or_null,
};

/// See also: [`IAudioEndpointVolume`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume)
#[derive(Debug, Clone)]
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
    pub fn get_channel_count(&self) -> windows::Result<u32> {
        unsafe { self.inner.GetChannelCount() }
    }

    /// See also: [`IAudioEndpointVolume::GetChannelVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getchannelvolumelevel)
    pub fn get_channel_volume_level(&self, channel: u32) -> windows::Result<f32> {
        unsafe { self.inner.GetChannelVolumeLevel(channel) }
    }

    /// See also: [`IAudioEndpointVolume::GetChannelVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getchannelvolumelevelscalar)
    pub fn get_channel_volume_level_scalar(&self, channel: u32) -> windows::Result<f32> {
        unsafe { self.inner.GetChannelVolumeLevelScalar(channel) }
    }

    /// See also: [`IAudioEndpointVolume::GetMasterVolumeLevel`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmastervolumelevel)
    pub fn get_master_volume_level(&self) -> windows::Result<f32> {
        unsafe { self.inner.GetMasterVolumeLevel() }
    }

    /// See also: [`IAudioEndpointVolume::GetMasterVolumeLevelScalar`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmastervolumelevelscalar)
    pub fn get_master_volume_level_scalar(&self) -> windows::Result<f32> {
        unsafe { self.inner.GetMasterVolumeLevelScalar() }
    }

    /// See also: [`IAudioEndpointVolume::GetMute`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getmute)
    pub fn get_mute(&self) -> windows::Result<bool> {
        unsafe { self.inner.GetMute().map(Into::into) }
    }

    /// See also: [`IAudioEndpointVolume::GetVolumeRange`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getvolumerange)
    pub fn get_volume_range(&self) -> windows::Result<VolumeRange> {
        let mut volume_range = VolumeRange {
            min_db: 0.0,
            max_db: 0.0,
            increment_db: 0.0,
        };
        unsafe {
            self.inner.GetVolumeRange(
                &mut volume_range.min_db,
                &mut volume_range.max_db,
                &mut volume_range.increment_db,
            )?
        };
        Ok(volume_range)
    }

    /// See also: [`IAudioEndpointVolume::GetVolumeStepInfo`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-getvolumestepinfo)
    pub fn get_volume_step_info(&self) -> windows::Result<VolumeStepInfo> {
        let mut volume_step_info = VolumeStepInfo {
            current_step: 0,
            num_steps: 0,
        };
        unsafe {
            self.inner.GetVolumeStepInfo(
                &mut volume_step_info.current_step,
                &mut volume_step_info.num_steps,
            )?
        };
        Ok(volume_step_info)
    }

    /// See also: [`IAudioEndpointVolume::QueryHardwareSupport`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-queryhardwaresupport)
    pub fn query_hardware_support(&self) -> windows::Result<HardwareSupportMask> {
        let raw = unsafe { self.inner.QueryHardwareSupport()? };
        Ok(HardwareSupportMask::from_bits(raw).expect("invalid mask"))
    }

    /// See also: [`IAudioEndpointVolume::RegisterControlChangeNotify`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nf-endpointvolume-iaudioendpointvolume-registercontrolchangenotify)
    pub fn register_control_change_notify<T>(
        &self,
        callback: T,
    ) -> windows::Result<AudioEndpointVolumeCallbackHandle>
    where
        T: AudioEndpointVolumeCallback,
    {
        let callback =
            IAudioEndpointVolumeCallback::from(AudioEndpointVolumeCallbackWrapper::new(callback));
        unsafe { self.inner.RegisterControlChangeNotify(&callback).unwrap() };
        Ok(AudioEndpointVolumeCallbackHandle {
            inner: callback,
            parent: self.inner.clone(),
        })
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct VolumeRange {
    pub min_db: f32,
    pub max_db: f32,
    pub increment_db: f32,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct VolumeStepInfo {
    pub current_step: u32,
    pub num_steps: u32,
}

#[derive(Debug, Clone)]
#[must_use = "callback will be unregistered when the handle is dropped"]
pub struct AudioEndpointVolumeCallbackHandle {
    inner: IAudioEndpointVolumeCallback,
    parent: IAudioEndpointVolume,
}

impl AudioEndpointVolumeCallbackHandle {
    pub fn unregister(self) {
        // Don't have to do anything, handled by the Drop impl
    }
}

impl Drop for AudioEndpointVolumeCallbackHandle {
    fn drop(&mut self) {
        unsafe { self.parent.UnregisterControlChangeNotify(&self.inner).ok() };
    }
}
