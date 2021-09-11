//! Owned and borrowed string types that are handled by the COM runtime.

use std::borrow::Borrow;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::fmt::{Formatter, Write};
use std::ops::Deref;

use crate::bindings::Windows::Win32::{
    Foundation::PWSTR,
    Globalization::lstrlenW,
    System::{Com::CoTaskMemFree, Memory::LocalFree},
    UI::Shell::StrDupW,
};

/// A borrowed string value that is valid only for the defined lifetime.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WinStr {
    slice: [u16],
}

impl WinStr {
    /// Wrap the given string pointer.
    ///
    /// # Safety
    ///
    /// - `pwstr` must point to a valid, null-terminated string.
    pub(crate) unsafe fn from_pwstr<'a>(pwstr: &'a PWSTR) -> &'a Self {
        let len = unsafe { lstrlenW(pwstr).try_into().expect("invalid string length") };
        let slice = unsafe { std::slice::from_raw_parts(pwstr.0 as *const u16, len) };
        unsafe { &*(slice as *const [u16] as *const Self) }
    }

    /// Gets a PWSTR pointer to the underlying string.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinStr` object is
    /// kept alive.
    ///
    /// The PWSTR pointer should not be used to mutate the string.
    pub(crate) fn as_pwstr(&self) -> PWSTR {
        PWSTR(self.slice.as_ptr() as *mut _)
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }

    pub fn as_slice(&self) -> &[u16] {
        &self.slice
    }

    /// Gets a raw pointer to the underlying wide string.
    ///
    /// # Safety
    ///
    /// This pointer will be valid as long as the parent `WinStr` object is
    /// kept alive.
    pub fn as_ptr(&self) -> *const u16 {
        self.slice.as_ptr()
    }

    pub fn to_winstring(&self) -> WinString {
        let pwstr = unsafe { StrDupW(self.as_pwstr()) };
        if pwstr.is_null() {
            panic!("unable to copy string");
        }
        unsafe { WinString::from_local_pwstr(pwstr) }
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

impl Debug for WinStr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char('"')?;
        for c in decode_utf16(self.slice.iter().copied())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .flat_map(|c| c.escape_debug())
        {
            f.write_char(c)?;
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl ToOwned for WinStr {
    type Owned = WinString;

    fn to_owned(&self) -> Self::Owned {
        self.to_winstring()
    }
}

impl PartialEq<WinString> for WinStr {
    fn eq(&self, other: &WinString) -> bool {
        self == other.as_winstr()
    }
}

impl PartialOrd<WinString> for WinStr {
    fn partial_cmp(&self, other: &WinString) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_winstr())
    }
}

/// An owned string value that will be automatically freed when dropped.
pub struct WinString {
    winstr: *const WinStr,
    alloc: StringAlloc,
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
    pub(crate) unsafe fn from_com_pwstr(pwstr: PWSTR) -> Self {
        Self {
            winstr: unsafe { WinStr::from_pwstr(&pwstr) },
            alloc: StringAlloc::Com,
        }
    }

    /// Wrap the given string pointer.
    ///
    /// # Safety
    ///
    /// - `pwstr` must point to a valid, null-terminated string.
    ///
    /// - Expect to call `LocalFree(pwstr)` when the string is no longer
    ///   used. This is usually the case for strings that are return values of
    /// API calls, and should be mentioned in the function documentation. For
    /// example, see [`StrDupW`](https://docs.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-strdupw).
    /// If this is not the case, create a borrowed [`WinStr`] instead.
    pub(crate) unsafe fn from_local_pwstr(pwstr: PWSTR) -> Self {
        Self {
            winstr: unsafe { WinStr::from_pwstr(&pwstr) },
            alloc: StringAlloc::Local,
        }
    }

    /// Borrow this string as a `WinStr`.
    pub fn as_winstr(&self) -> &WinStr {
        unsafe { &*self.winstr }
    }
}

impl Deref for WinString {
    type Target = WinStr;

    fn deref(&self) -> &Self::Target {
        self.as_winstr()
    }
}

impl AsRef<WinStr> for WinString {
    fn as_ref(&self) -> &WinStr {
        self.as_winstr()
    }
}

impl Borrow<WinStr> for WinString {
    fn borrow(&self) -> &WinStr {
        self.as_winstr()
    }
}

impl Debug for WinString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_winstr())
    }
}

impl Clone for WinString {
    fn clone(&self) -> Self {
        self.as_winstr().to_winstring()
    }
}

impl PartialEq for WinString {
    fn eq(&self, other: &Self) -> bool {
        self.as_winstr() == other.as_winstr()
    }
}

impl PartialEq<WinStr> for WinString {
    fn eq(&self, other: &WinStr) -> bool {
        self.as_winstr() == other
    }
}

impl Eq for WinString {}

impl PartialOrd for WinString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_winstr().partial_cmp(other.as_winstr())
    }
}

impl PartialOrd<WinStr> for WinString {
    fn partial_cmp(&self, other: &WinStr) -> Option<std::cmp::Ordering> {
        self.as_winstr().partial_cmp(other)
    }
}

impl Ord for WinString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_winstr().cmp(other.as_winstr())
    }
}

impl Drop for WinString {
    fn drop(&mut self) {
        match &self.alloc {
            StringAlloc::Com => unsafe {
                CoTaskMemFree(self.as_pwstr().0 as _);
            },
            StringAlloc::Local => unsafe {
                LocalFree(self.as_pwstr().0 as _);
            },
        }
    }
}

// Safety: The pointer contained by a WinString is guaranteed to be unique.
unsafe impl Send for WinString {}
unsafe impl Sync for WinString {}

#[derive(Debug)]
enum StringAlloc {
    Com,
    Local,
}
