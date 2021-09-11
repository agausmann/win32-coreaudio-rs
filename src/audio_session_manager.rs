use std::ops::Deref;

use windows::{Guid, Interface};

use crate::{
    audio_session_control::AudioSessionControl,
    audio_session_enumerator::AudioSessionEnumerator,
    audio_session_notification::{AudioSessionNotification, AudioSessionNotificationWrapper},
    audio_volume_duck_notification::{
        AudioVolumeDuckNotification, AudioVolumeDuckNotificationWrapper,
    },
    bindings::Windows::Win32::Media::Audio::CoreAudio::{
        IAudioSessionManager, IAudioSessionManager2, IAudioSessionNotification,
        IAudioVolumeDuckNotification,
    },
    device::Activate,
    simple_audio_volume::SimpleAudioVolume,
    string::WinStr,
};

/// See also: [`IAudioSessionManager`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionmanager)
#[derive(Debug, Clone)]
pub struct AudioSessionManager {
    inner: IAudioSessionManager,
}

impl Activate for AudioSessionManager {
    type Raw = IAudioSessionManager;

    fn from_raw(inner: Self::Raw) -> Self {
        Self { inner }
    }
}

impl AudioSessionManager {
    /// See also: [`IAudioSessionManager::GetAudioSessionControl`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager-getaudiosessioncontrol)
    pub fn get_audio_session_control(
        &self,
        audio_session_guid: &Guid,
    ) -> windows::Result<AudioSessionControl> {
        unsafe {
            self.inner
                .GetAudioSessionControl(audio_session_guid, 0)
                .map(AudioSessionControl::new)
        }
    }

    /// See also: [`IAudioSessionManager::GetSimpleAudioVolume`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager-getsimpleaudiovolume)
    pub fn get_simple_audio_volume(
        &self,
        audio_session_guid: &Guid,
    ) -> windows::Result<SimpleAudioVolume> {
        unsafe {
            self.inner
                .GetSimpleAudioVolume(audio_session_guid, 0)
                .map(SimpleAudioVolume::new)
        }
    }

    pub fn upgrade(&self) -> windows::Result<AudioSessionManager2> {
        self.inner.cast().map(AudioSessionManager2::from_raw)
    }
}

/// See also: [`IAudioSessionManager2`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionmanager2)
#[derive(Debug, Clone)]
pub struct AudioSessionManager2 {
    inner: IAudioSessionManager2,
    downgrade: AudioSessionManager,
}

impl Activate for AudioSessionManager2 {
    type Raw = IAudioSessionManager2;

    fn from_raw(inner: Self::Raw) -> Self {
        let downgrade = AudioSessionManager::from_raw(inner.cast().unwrap());
        Self { inner, downgrade }
    }
}

impl AudioSessionManager2 {
    /// See also: [`IAudioSessionManager2::GetSessionEnumerator`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager2-getsessionenumerator)
    pub fn get_session_enumerator(&self) -> windows::Result<AudioSessionEnumerator> {
        unsafe {
            self.inner
                .GetSessionEnumerator()
                .map(AudioSessionEnumerator::new)
        }
    }

    /// See also: [`IAudioSessionManager2::RegisterDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager2-registerducknotification)
    pub fn register_duck_notification<T>(
        &self,
        session_id: &WinStr,
        duck_notification: T,
    ) -> windows::Result<AudioVolumeDuckNotificationHandle>
    where
        T: AudioVolumeDuckNotification,
    {
        let duck_notification = IAudioVolumeDuckNotification::from(
            AudioVolumeDuckNotificationWrapper::new(duck_notification),
        );
        unsafe {
            self.inner
                .RegisterDuckNotification(session_id.as_pwstr(), &duck_notification)?
        };
        Ok(AudioVolumeDuckNotificationHandle {
            inner: duck_notification,
        })
    }

    /// See also: [`IAudioSessionManager2::RegisterSessionNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager2-registersessionnotification)
    pub fn register_session_notification<T>(
        &self,
        session_notification: T,
    ) -> windows::Result<AudioSessionNotificationHandle>
    where
        T: AudioSessionNotification,
    {
        let session_notification = IAudioSessionNotification::from(
            AudioSessionNotificationWrapper::new(session_notification),
        );
        unsafe {
            self.inner
                .RegisterSessionNotification(&session_notification)?;
        }
        Ok(AudioSessionNotificationHandle {
            inner: session_notification,
        })
    }

    /// See also: [`IAudioSessionManager2::UnregisterDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager2-unregisterducknotification)
    pub fn unregister_duck_notification(
        &self,
        handle: &AudioVolumeDuckNotificationHandle,
    ) -> windows::Result<()> {
        unsafe { self.inner.UnregisterDuckNotification(&handle.inner) }
    }

    /// See also: [`IAudioSessionManager2::UnregisterSessionNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionmanager2-unregistersessionnotification)
    pub fn unregister_session_notification(
        &self,
        handle: &AudioSessionNotificationHandle,
    ) -> windows::Result<()> {
        unsafe { self.inner.UnregisterSessionNotification(&handle.inner) }
    }
}

impl Deref for AudioSessionManager2 {
    type Target = AudioSessionManager;

    fn deref(&self) -> &Self::Target {
        &self.downgrade
    }
}

#[derive(Debug, Clone)]
pub struct AudioVolumeDuckNotificationHandle {
    inner: IAudioVolumeDuckNotification,
}

#[derive(Debug, Clone)]
pub struct AudioSessionNotificationHandle {
    inner: IAudioSessionNotification,
}
