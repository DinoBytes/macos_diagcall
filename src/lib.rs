#![feature(asm)]

#[cfg(any(target_arch = "x86_64"))]
fn diag_call(diag_id: i32, buf: &Vec<u32>) -> u32 {
    let ret: u32;
    let mode = 0x4000001; // syscall value for specifying a diagnostic call

    let ptr = buf.as_ptr();

    unsafe {
        asm!(
            // macOS does not export a public wrapper for diagnostic calls
            // so 'syscall' must be executed manually
            "syscall" :

            // Return value of the syscall is found in eax.
            // 1 means success, 0 means failure
            "={eax}"(ret) :

            // eax contains the diagnostic call
            // rdi contains the diagnostic id
            // rsi contains the starting address of the buffer to write to
            "{eax}"(mode), "{rdi}"(diag_id), "{rsi}"(ptr) :

            // syscall clobbers rcx, r11, cc, and memory
            // rcx contains return address of next instruction
            // r11 is a temprary store for rflags
            // cc is a temporary store for condition codes
            // memory indicates that this call has written to a buffer in memory (buf)
            "rcx", "r11", "cc", "memory" :
            
            // intel syntax, avoid some compiler optimizations
            "intel", "volatile"
        );
    }

    if ret != 1 && ret != 0 {
        panic!("Something went very wrong.");
    }

    ret
}

pub fn dg_rupt_stat() -> Result<Vec<u32>, &'static str> {
    let buffer_size = 5000;
    let buf = vec![0u32; buffer_size];

    let diag_result = diag_call(25, &buf);

    if diag_result == 1 {
        Ok(buf)
    } else {
        Err("Diagnostic Call failed.")
    }
}

pub fn dg_power_stat() -> Result<Vec<u32>, &'static str> {
    let buffer_size = 10000;
    let buf = vec![0u32; buffer_size];

    let diag_result = diag_call(17, &buf);

    if diag_result == 1 {
        Ok(buf)
    } else {
        Err("Diagnostic Call failed.")
    }
}
