use windows::Guid;

use crate::{
    bindings::Windows::Win32::Media::Audio::CoreAudio::ISimpleAudioVolume, util::as_raw_or_null,
};

/// See also: [`ISimpleAudioVolume`](https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nn-audioclient-isimpleaudiovolume)
#[derive(Debug, Clone)]
pub struct SimpleAudioVolume {
    inner: ISimpleAudioVolume,
}

impl SimpleAudioVolume {
    pub(crate) fn new(inner: ISimpleAudioVolume) -> Self {
        Self { inner }
    }

    /// See also: [`ISimpleAudioVolume::GetMasterVolume`](https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-isimpleaudiovolume-getmastervolume)
    pub fn get_master_volume(&self) -> windows::Result<f32> {
        unsafe { self.inner.GetMasterVolume() }
    }

    /// See also: [`ISimpleAudioVolume::GetMute`](https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-isimpleaudiovolume-getmute)
    pub fn get_mute(&self) -> windows::Result<bool> {
        unsafe { self.inner.GetMute().map(Into::into) }
    }

    /// See also: [`ISimpleAudioVolume::SetMasterVolume`](https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-isimpleaudiovolume-setmastervolume)
    pub fn set_master_volume(
        &self,
        volume_level: f32,
        event_context: Option<&Guid>,
    ) -> windows::Result<()> {
        unsafe {
            self.inner
                .SetMasterVolume(volume_level, as_raw_or_null(event_context))
        }
    }

    /// See also: [`ISimpleAudioVolume::SetMute`](https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-isimpleaudiovolume-setmute)
    pub fn set_mute(&self, mute: bool, event_context: Option<&Guid>) -> windows::Result<()> {
        unsafe { self.inner.SetMute(mute, as_raw_or_null(event_context)) }
    }
}
