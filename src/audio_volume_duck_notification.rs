use crate::bindings::Windows::Win32::Foundation::PWSTR;
use crate::bindings::*;

use crate::string::WinStr;

/// See also: [`IAudioVolumeDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiovolumeducknotification)
pub trait AudioVolumeDuckNotification: 'static {
    /// See also: [`IAudioVolumeDuckNotification::OnVolumeDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiovolumeducknotification-onvolumeducknotification)
    fn on_volume_duck_notification(
        &mut self,
        session_id: &WinStr,
        num_communication_sessions: u32,
    ) -> windows::Result<()> {
        let _ = (session_id, num_communication_sessions);
        Ok(())
    }

    /// See also: [`IAudioVolumeDuckNotification::OnVolumeUnduckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiovolumeducknotification-onvolumeunducknotification)
    fn on_volume_unduck_notification(&mut self, session_id: &WinStr) -> windows::Result<()> {
        let _ = session_id;
        Ok(())
    }
}

#[windows::implement(Windows::Win32::Media::Audio::CoreAudio::IAudioVolumeDuckNotification)]
pub(crate) struct AudioVolumeDuckNotificationWrapper {
    inner: Box<dyn AudioVolumeDuckNotification>,
}

impl AudioVolumeDuckNotificationWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioVolumeDuckNotification,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioVolumeDuckNotification
#[allow(non_snake_case)]
impl AudioVolumeDuckNotificationWrapper {
    fn OnVolumeDuckNotification(
        &mut self,
        session_id: PWSTR,
        num_communication_sessions: u32,
    ) -> windows::Result<()> {
        self.inner.on_volume_duck_notification(
            unsafe { WinStr::from_pwstr(&session_id) },
            num_communication_sessions,
        )
    }

    fn OnVolumeUnduckNotification(&mut self, session_id: PWSTR) -> windows::Result<()> {
        self.inner
            .on_volume_unduck_notification(unsafe { WinStr::from_pwstr(&session_id) })
    }
}
