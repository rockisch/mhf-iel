use windows::{
    core::HSTRING,
    Win32::{Foundation::HANDLE, System::Threading::CreateMutexW},
};

use crate::{Error, Result};

pub fn bufcopy<T: Copy>(s: &mut [T], v: &[T]) {
    let l = s.len().min(v.len());
    s[..l].copy_from_slice(&v[..l])
}

pub fn get_mutex_name(s: &str) -> String {
    format!("Monster Hunter Frontier Z {s}")
}

pub fn create_mutex(name: impl Into<HSTRING>) -> Result<HANDLE> {
    unsafe { CreateMutexW(None, false, &name.into()) }.or(Err(Error::Mutex))
}

// fn get_mutex(name: impl Into<HSTRING>) -> Result<HANDLE> {
//     unsafe { OpenMutexW(SYNCHRONIZATION_ACCESS_RIGHTS(0x1F0001), false, &name.into()) }
//         .or(Err(Error::Mutex))
// }
