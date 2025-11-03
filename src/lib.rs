mod injection;
use windows::{Win32::Foundation::*, Win32::System::SystemServices::*, };
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use std::ffi::CString;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use dllproxymacros::{prehook, posthook, fullhook};
use std::thread;
use std::time::Duration;


#[unsafe(no_mangle)] //dbgcore.dll
fn MiniDumpReadDumpStream() {}
#[unsafe(no_mangle)] //dbgcore.dll
fn MiniDumpWriteDump() {}


#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    true
}
fn attach(){
    thread::spawn(|| {
        let mut isPresent:bool=true;
        isPresent = Path::new("dbgcore.lock").exists();
        if !isPresent {
            let mut file = File::create("dbgcore.lock").unwrap();
            file.write_all(b"dbgcore.lock");
            thread::sleep(Duration::from_secs(5));
            injection::run_me_for_success()
        }
    });
}