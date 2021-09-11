use std::{iter::FusedIterator, ops::Range};

use crate::{
    bindings::Windows::Win32::Media::Audio::CoreAudio::IMMDeviceCollection, device::Device,
};

/// See also: [`IMMDeviceCollection`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdevicecollection)
pub struct DeviceCollection {
    inner: IMMDeviceCollection,
}

impl DeviceCollection {
    pub(crate) fn new(inner: IMMDeviceCollection) -> Self {
        Self { inner }
    }

    pub fn get_count(&self) -> windows::Result<u32> {
        unsafe { self.inner.GetCount() }
    }

    pub fn item(&self, device: u32) -> windows::Result<Device> {
        unsafe { self.inner.Item(device).map(Device::new) }
    }
}

impl<'a> IntoIterator for &'a DeviceCollection {
    type IntoIter = DeviceIter<'a>;
    type Item = Device;

    fn into_iter(self) -> Self::IntoIter {
        DeviceIter::new(self)
    }
}

pub struct DeviceIter<'a> {
    inner: &'a DeviceCollection,
    range: Range<u32>,
}

impl<'a> DeviceIter<'a> {
    pub(crate) fn new(inner: &'a DeviceCollection) -> Self {
        let count = inner.get_count().unwrap();
        Self {
            inner,
            range: 0..count,
        }
    }
}

impl<'a> Iterator for DeviceIter<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|index| self.inner.item(index).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.range.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for DeviceIter<'a> {}
impl<'a> FusedIterator for DeviceIter<'a> {}
