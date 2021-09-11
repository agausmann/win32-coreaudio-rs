use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::IAudioSessionEnumerator;

/// See also: [`IAudioSessionEnumerator`](https://docs.microsoft.com/en-us/windows/desktop/api/audiopolicy/nn-audiopolicy-iaudiosessionenumerator)
pub struct AudioSessionEnumerator {
    inner: IAudioSessionEnumerator,
}

impl AudioSessionEnumerator {
    pub fn new(inner: IAudioSessionEnumerator) -> Self {
        Self { inner }
    }
}
