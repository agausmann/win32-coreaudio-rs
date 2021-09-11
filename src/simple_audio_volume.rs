use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::ISimpleAudioVolume;

pub struct SimpleAudioVolume {
    inner: ISimpleAudioVolume,
}

impl SimpleAudioVolume {
    pub(crate) fn new(inner: ISimpleAudioVolume) -> Self {
        Self { inner }
    }
}
