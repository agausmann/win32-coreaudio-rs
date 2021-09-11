use std::{iter::FusedIterator, ops::Range};

use crate::{
    audio_session_control::AudioSessionControl,
    bindings::Windows::Win32::Media::Audio::CoreAudio::IAudioSessionEnumerator,
};

/// See also: [`IAudioSessionEnumerator`](https://docs.microsoft.com/en-us/windows/desktop/api/audiopolicy/nn-audiopolicy-iaudiosessionenumerator)
pub struct AudioSessionEnumerator {
    inner: IAudioSessionEnumerator,
}

impl AudioSessionEnumerator {
    pub(crate) fn new(inner: IAudioSessionEnumerator) -> Self {
        Self { inner }
    }

    pub fn get_count(&self) -> windows::Result<i32> {
        unsafe { self.inner.GetCount() }
    }

    pub fn get_session(&self, session: i32) -> windows::Result<AudioSessionControl> {
        unsafe { self.inner.GetSession(session).map(AudioSessionControl::new) }
    }
}

impl<'a> IntoIterator for &'a AudioSessionEnumerator {
    type IntoIter = Iter<'a>;
    type Item = AudioSessionControl;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter<'a> {
    inner: &'a AudioSessionEnumerator,
    range: Range<i32>,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(inner: &'a AudioSessionEnumerator) -> Self {
        let count = inner.get_count().unwrap();
        Self {
            inner,
            range: 0..count,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = AudioSessionControl;

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|index| self.inner.get_session(index).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.range.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}
impl<'a> FusedIterator for Iter<'a> {}
