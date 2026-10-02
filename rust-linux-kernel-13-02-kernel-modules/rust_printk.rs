// SPDX-License-Identifier: GPL-2.0

//! 러스트 FFI 예제

use kernel::prelude::*;
use kernel::ffi;

// 커널 함수 `printk`를 위한 FFI 선언
unsafe extern "C" {
    fn _printk(fmt: *const ffi::c_char, ...) -> ffi::c_int;
}

fn rust_printk(message: &str) {
    // Rust 문자열을 C 문자열로 변환
    let cstr = kernel::str::CString::try_from_fmt(fmt!("{message}")).unwrap();

    // `printk` 함수 호출을 위해 `unsafe` 블록 사용
    unsafe {
        _printk(cstr.as_char_ptr());
    }
}

struct RustKernelPrintkModule;

impl kernel::Module for RustKernelPrintkModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        rust_printk("Hello, Rust kernel module!\n");
        Ok(RustKernelPrintkModule)
    }
}

module! {
    type: RustKernelPrintkModule,
    name: "rust_kernel_printk",
    authors: ["Rust for Linux Contributors"],
    description: "Rust kernel module using printk",
    license: "GPL",
}
