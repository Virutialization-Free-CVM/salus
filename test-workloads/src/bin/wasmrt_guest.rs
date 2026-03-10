// SPDX-FileCopyrightText: 2026 The Salus Contributors
//
// SPDX-License-Identifier: Apache-2.0

#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![allow(missing_docs)]

use core::alloc::{GlobalAlloc, Layout};

extern crate test_workloads;

use liberum_wasmrt_guest::{run_wasmrt_guest, secondary_guest_init};
use s_mode_utils::abort::abort;

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
extern "C" fn kernel_init(_hart_id: u64, boot_args: u64) {
    run_wasmrt_guest(boot_args)
}

#[no_mangle]
extern "C" fn secondary_init(_hart_id: u64) {
    secondary_guest_init()
}
