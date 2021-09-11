use crate::{
    bindings::Windows::Win32::{
        Storage::StructuredStorage::{
            PROPVARIANT_0_0_0_abi, PROPVARIANT_0_0_abi, PROPVARIANT, PROPVARIANT_0,
        },
        System::{
            OleAutomation::{
                VARENUM, VT_BOOL, VT_EMPTY, VT_I1, VT_I2, VT_I4, VT_I8, VT_INT, VT_LPWSTR, VT_NULL,
                VT_R4, VT_R8, VT_UI1, VT_UI2, VT_UI4, VT_UI8, VT_UINT,
            },
            PropertiesSystem::{IPropertyStore, PropVariantToStringAlloc, PROPERTYKEY},
            SystemServices::CHAR,
        },
    },
    string::WinString,
};

#[derive(Debug, Clone, Copy)]
pub struct PropertyKey(PROPERTYKEY);

impl PropertyKey {
    pub(crate) const fn from_raw(raw: PROPERTYKEY) -> Self {
        Self(raw)
    }

    pub(crate) fn as_raw(&self) -> &PROPERTYKEY {
        &self.0
    }
}

/// See also: [`PROPVARIANT`](https://docs.microsoft.com/en-us/windows/win32/api/propidlbase/ns-propidlbase-propvariant)
#[derive(Debug, Clone)]
pub enum Property {
    Empty,
    Null,
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(WinString),
    /// The property contains a type that is not yet supported by this crate.
    Unsupported,
}

impl Property {
    pub(crate) fn from_raw(raw: PROPVARIANT) -> Self {
        let raw_inner = unsafe { raw.Anonymous.Anonymous };
        let tag = VARENUM(raw_inner.vt as _);
        let value = raw_inner.Anonymous;
        if tag == VT_EMPTY {
            Self::Empty
        } else if tag == VT_NULL {
            Self::Null
        } else if tag == VT_I1 {
            Self::I8(unsafe { value.cVal.0 as _ })
        } else if tag == VT_UI1 {
            Self::U8(unsafe { value.bVal })
        } else if tag == VT_I2 {
            Self::I16(unsafe { value.iVal })
        } else if tag == VT_UI2 {
            Self::U16(unsafe { value.uiVal })
        } else if tag == VT_I4 {
            Self::I32(unsafe { value.lVal })
        } else if tag == VT_UI4 {
            Self::U32(unsafe { value.ulVal })
        } else if tag == VT_INT {
            Self::I32(unsafe { value.intVal })
        } else if tag == VT_UINT {
            Self::U32(unsafe { value.uintVal })
        } else if tag == VT_I8 {
            Self::I64(unsafe { value.hVal })
        } else if tag == VT_UI8 {
            Self::U64(unsafe { value.uhVal })
        } else if tag == VT_R4 {
            Self::F32(unsafe { value.fltVal })
        } else if tag == VT_R8 {
            Self::F64(unsafe { value.dblVal })
        } else if tag == VT_BOOL {
            Self::Bool(unsafe { value.boolVal != 0 })
        } else if tag == VT_LPWSTR {
            Self::Str(unsafe { WinString::from_com_pwstr(PropVariantToStringAlloc(&raw).unwrap()) })
        } else {
            Self::Unsupported
        }
    }

    pub(crate) fn to_raw(&self) -> PROPVARIANT {
        type Data = PROPVARIANT_0_0_0_abi;
        let (tag, data) = match self {
            Property::Empty => (VT_EMPTY, Data { bVal: 0 }),
            Property::Null => (VT_NULL, Data { bVal: 0 }),
            Property::I8(x) => (
                VT_I1,
                Data {
                    cVal: CHAR(*x as _),
                },
            ),
            Property::U8(x) => (VT_UI1, Data { bVal: *x }),
            Property::I16(x) => (VT_I2, Data { iVal: *x }),
            Property::U16(x) => (VT_UI2, Data { uiVal: *x }),
            Property::I32(x) => (VT_I4, Data { lVal: *x }),
            Property::U32(x) => (VT_UI4, Data { ulVal: *x }),
            Property::I64(x) => (VT_I8, Data { hVal: *x }),
            Property::U64(x) => (VT_UI8, Data { uhVal: *x }),
            Property::F32(x) => (VT_R4, Data { fltVal: *x }),
            Property::F64(x) => (VT_R8, Data { dblVal: *x }),
            Property::Bool(x) => (VT_BOOL, Data { boolVal: *x as _ }),
            Property::Str(x) => (
                VT_LPWSTR,
                Data {
                    pwszVal: x.as_pwstr(),
                },
            ),
            Property::Unsupported => panic!("cannot convert unsupported"),
        };
        PROPVARIANT {
            Anonymous: PROPVARIANT_0 {
                Anonymous: PROPVARIANT_0_0_abi {
                    vt: tag.0 as _,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: data,
                },
            },
        }
    }
}

/// See also: [`IPropertyStore`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nn-propsys-ipropertystore)
#[derive(Debug, Clone)]
pub struct PropertyStore {
    inner: IPropertyStore,
}

impl PropertyStore {
    pub(crate) fn new(inner: IPropertyStore) -> Self {
        Self { inner }
    }

    /// See also: [`IPropertyStore::Commit`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-commit)
    pub fn commit(&self) -> windows::Result<()> {
        unsafe { self.inner.Commit() }
    }

    /// See also: [`IPropertyStore::GetAt`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getat)
    pub fn get_at(&self, index: u32) -> windows::Result<PropertyKey> {
        unsafe { self.inner.GetAt(index).map(PropertyKey::from_raw) }
    }

    /// See also: [`IPropertyStore::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getcount)
    pub fn get_count(&self) -> windows::Result<u32> {
        unsafe { self.inner.GetCount() }
    }

    /// See also: [`IPropertyStore::GetValue`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getvalue)
    pub fn get_value(&self, key: &PropertyKey) -> windows::Result<Property> {
        unsafe { self.inner.GetValue(key.as_raw()).map(Property::from_raw) }
    }

    /// See also: [`IPropertyStore::SetValue`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-setvalue)
    pub fn set_value(&self, key: &PropertyKey, property: &Property) -> windows::Result<()> {
        unsafe { self.inner.SetValue(key.as_raw(), &property.to_raw()) }
    }
}
