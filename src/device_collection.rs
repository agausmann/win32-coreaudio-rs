use std::{convert::TryInto, iter::FusedIterator};

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
}

impl<'a> IntoIterator for &'a DeviceCollection {
    type IntoIter = Iter<'a>;
    type Item = Device;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(&self.inner)
    }
}

pub struct Iter<'a> {
    inner: &'a IMMDeviceCollection,
    count: u32,
    next_index: u32,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(inner: &'a IMMDeviceCollection) -> Self {
        let count = unsafe { inner.GetCount().unwrap() };
        Self {
            inner,
            count,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index < self.count {
            let device = unsafe { self.inner.Item(self.next_index).unwrap() };
            self.next_index += 1;
            Some(Device::new(device))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.count - self.next_index)
            .try_into()
            .expect("size out of bounds");
        (size, Some(size))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}
impl<'a> FusedIterator for Iter<'a> {}
