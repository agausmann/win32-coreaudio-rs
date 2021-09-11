use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::IAudioSessionControl;

pub struct AudioSessionControl {
    inner: IAudioSessionControl,
}

impl AudioSessionControl {
    pub(crate) fn new(inner: IAudioSessionControl) -> Self {
        Self { inner }
    }
}
