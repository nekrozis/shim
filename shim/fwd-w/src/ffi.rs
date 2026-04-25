#![allow(non_camel_case_types)]

type HANDLE = *mut core::ffi::c_void;
type FILE_TYPE = u32;
pub const FILE_TYPE_UNKNOWN: FILE_TYPE = 0u32;

windows_targets::link!("kernel32.dll" "system" fn GetFileType(hfile : HANDLE) -> FILE_TYPE);
