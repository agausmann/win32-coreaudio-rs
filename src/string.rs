//! Owned and borrowed string types that are handled by the COM runtime.

use crate::bindings::Windows::Win32::{Foundation::PWSTR, System::Com::CoTaskMemFree};

/// A borrowed string value that is valid only for the defined lifetime.
#[derive(Debug, Clone)]
pub struct WinStr<'a> {
    pwstr: &'a PWSTR,
}

impl<'a> WinStr<'a> {
    /// Wrap the given string pointer.
    ///
    /// # Safety
    ///
    /// - `pwstr` must point to a valid, null-terminated string.
    pub(crate) unsafe fn from_pwstr(pwstr: &'a PWSTR) -> Self {
        Self { pwstr }
    }

    /// Gets a raw pointer to the underlying wide string.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinStr` object is
    /// kept alive.
    pub fn as_raw(&self) -> *const u16 {
        self.pwstr.0 as *const _
    }

    /// Gets a copy of the wrapped PWSTR pointer.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinStr` object is
    /// kept alive.
    pub(crate) fn to_pwstr(&self) -> PWSTR {
        *self.pwstr
    }
}

/// An owned string value that will be automatically freed when dropped.
#[derive(Debug)]
pub struct WinString {
    pwstr: PWSTR,
}

impl WinString {
    /// Wrap the given string pointer.
    ///
    /// # Safety
    ///
    /// - `pwstr` must point to a valid, null-terminated string.
    ///
    /// - Expect to call `CoTaskMemFree(pwstr)` when the string is no longer
    ///   used. This is usually the case for strings that are return values of
    /// API calls, and should be mentioned in the function documentation. For
    /// example, see [`IMMDevice::GetId`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getid).
    /// If this is not the case, create a borrowed [`WinStr`] instead.
    pub(crate) unsafe fn from_pwstr(pwstr: PWSTR) -> Self {
        Self { pwstr }
    }

    /// Borrow this string as a `WinStr`.
    pub fn as_str(&self) -> WinStr {
        WinStr { pwstr: &self.pwstr }
    }

    /// Gets a raw pointer to the underlying wide string.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinString` object is
    /// kept alive.
    pub fn as_raw(&self) -> *const u16 {
        self.pwstr.0 as *const _
    }

    /// Gets a copy of the wrapped PWSTR pointer.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinStr` object is
    /// kept alive.
    pub(crate) fn to_pwstr(&self) -> PWSTR {
        self.pwstr
    }
}

impl Drop for WinString {
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(self.pwstr.0 as _) }
    }
}
