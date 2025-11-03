## RUST DLL 


This project is based on https://github.com/tehstoni/RustHarder (all credits for the original code)

### How To:
- Install rustdllproxy create:
```
cargo install rustdllproxy
```
- Copy your intended DLL to a folder:
```
 rustdllproxy.exe -p dbgcore.dll -o VersionDllCrate -n VersionDllProxy
```
- This will create a new folder VersionDllProxy containing the exports from the original DLL
- Copy the exports from VersionDllProxy/VersionDllProxy/src/lib.rs as in the example below
```rust
#[no_mangle] //version.dll
fn GetFileVersionInfoA() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoByHandle() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoExA() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoExW() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoSizeA() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoSizeExA() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoSizeExW() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoSizeW() {}
#[no_mangle] //version.dll
fn GetFileVersionInfoW() {}
#[no_mangle] //version.dll
fn VerFindFileA() {}
#[no_mangle] //version.dll
fn VerFindFileW() {}
#[no_mangle] //version.dll
fn VerInstallFileA() {}
#[no_mangle] //version.dll
fn VerInstallFileW() {}
#[no_mangle] //version.dll
fn VerLanguageNameA() {}
#[no_mangle] //version.dll
fn VerLanguageNameW() {}
#[no_mangle] //version.dll
fn VerQueryValueA() {}
#[no_mangle] //version.dll
fn VerQueryValueW() {}
```
- Replace our own code in lib.rs lines 13 to 16:
```rust
#[unsafe(no_mangle)] //dbgcore.dll
fn MiniDumpReadDumpStream() {}
#[unsafe(no_mangle)] //dbgcore.dll
fn MiniDumpWriteDump() {}
```
- Build and place the dll in the folder with your application


