use crate::bindings::Windows::Win32::System::PropertiesSystem::IPropertyStore;

pub struct PropertyStore {
    inner: IPropertyStore,
}

impl PropertyStore {
    pub(crate) fn new(inner: IPropertyStore) -> Self {
        Self { inner }
    }
}
