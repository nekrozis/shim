#![allow(non_camel_case_types)]

pub type BOOL = i32;
pub const TRUE: BOOL = 1i32;
// pub const FALSE: BOOL = 0i32;

pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> BOOL>;

windows_targets::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : BOOL) -> BOOL);
