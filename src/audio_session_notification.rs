use crate::audio_session_control::AudioSessionControl;
use crate::bindings::Windows::Win32::Media::Audio::CoreAudio::IAudioSessionControl;
use crate::bindings::*;

/// See also: [`IAudioSessionNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionnotification)
pub trait AudioSessionNotification: 'static {
    /// See also: [`IAudioSessionNotification::OnSessionCreated`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionnotification-onsessioncreated)
    fn on_session_created(&mut self, new_session: AudioSessionControl) -> windows::Result<()> {
        let _ = new_session;
        Ok(())
    }
}

#[windows::implement(Windows::Win32::Media::Audio::CoreAudio::IAudioSessionNotification)]
pub(crate) struct AudioSessionNotificationWrapper {
    inner: Box<dyn AudioSessionNotification>,
}

impl AudioSessionNotificationWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioSessionNotification,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioSessionNotification
#[allow(non_snake_case)]
impl AudioSessionNotificationWrapper {
    fn OnSessionCreated(
        &mut self,
        new_session: &Option<IAudioSessionControl>,
    ) -> windows::Result<()> {
        self.inner
            .on_session_created(AudioSessionControl::new(new_session.clone().unwrap()))
    }
}
