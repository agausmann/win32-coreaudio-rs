pub(crate) fn as_raw_or_null<T>(option: Option<&T>) -> *const T {
    option.map(|x| x as *const _).unwrap_or(std::ptr::null())
}
