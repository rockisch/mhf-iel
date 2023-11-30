use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GlobalFree, HANDLE, HGLOBAL},
        System::{
            Memory::{GlobalAlloc, GLOBAL_ALLOC_FLAGS},
            Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        },
    },
};

use crate::{Error, Result};

pub fn bufcopy<T: Copy>(s: &mut [T], v: &[T]) {
    let l = s.len().min(v.len());
    s[..l].copy_from_slice(&v[..l])
}

pub fn get_mutex_name(s: &str) -> String {
    let pid = std::process::id();
    // F5 uses 'Monster Hunter Frontier Online', but it's probably fine
    format!("Monster Hunter Frontier Z {s} {pid}")
}

pub fn create_mutex(name: impl Into<HSTRING>) -> Result<HANDLE> {
    unsafe { CreateMutexW(None, false, &name.into()) }.or(Err(Error::Mutex))
}

pub fn get_or_create_mutex(name: impl Into<HSTRING> + Copy) -> Result<HANDLE> {
    unsafe { OpenMutexW(SYNCHRONIZATION_ACCESS_RIGHTS(0x1F0001), false, &name.into()) }
        .or_else(|_| create_mutex(name))
        .or(Err(Error::Mutex))
}

// pub fn release_mutex(handle: HANDLE) -> Result<()> {
//     unsafe { ReleaseMutex(handle) }.or(Err(Error::Mutex))
// }

pub fn create_global_alloc() -> Result<HGLOBAL> {
    unsafe { GlobalAlloc(GLOBAL_ALLOC_FLAGS(0x42), 0x8ae0) }.or(Err(Error::GlobalAlloc))
}

pub fn release_global_alloc(handle: HGLOBAL) -> Result<HGLOBAL> {
    unsafe { GlobalFree(handle) }.or(Err(Error::GlobalAlloc))
}
