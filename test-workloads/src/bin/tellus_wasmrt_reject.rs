// SPDX-FileCopyrightText: 2026 The Salus Contributors
//
// SPDX-License-Identifier: Apache-2.0

#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![allow(missing_docs)]

use core::alloc::{GlobalAlloc, Layout};

use liberum_tellus_wasmrt::run_tellus_wasmrt_host;
use s_mode_utils::abort::abort;

const CONSOLE_BUFFER_SIZE: usize = 256;
static mut CONSOLE_BUFFER: [u8; CONSOLE_BUFFER_SIZE] = [0; CONSOLE_BUFFER_SIZE];

struct GeneralGlobalAlloc;

unsafe impl GlobalAlloc for GeneralGlobalAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        abort()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        abort()
    }
}

#[global_allocator]
static GENERAL_ALLOCATOR: GeneralGlobalAlloc = GeneralGlobalAlloc;

#[alloc_error_handler]
pub fn alloc_error(_layout: Layout) -> ! {
    abort()
}

#[no_mangle]
extern "C" fn kernel_init(_hart_id: u64, fdt_addr: u64) {
    unsafe {
        let console_mem =
            core::slice::from_raw_parts_mut(&raw mut CONSOLE_BUFFER as *mut u8, CONSOLE_BUFFER_SIZE);
        run_tellus_wasmrt_host(console_mem, fdt_addr, 1);
    }
}

#[no_mangle]
extern "C" fn secondary_init(_hart_id: u64) {
    abort()
}
