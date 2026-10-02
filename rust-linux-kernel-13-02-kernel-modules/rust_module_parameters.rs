// SPDX-License-Identifier: GPL-2.0

//! Rust module parameters sample.

use kernel::prelude::*;

module! {
    type: RustModuleParameters,
    name: "rust_module_parameters",
    authors: ["Rust for Linux Contributors"],
    description: "Rust module parameters sample",
    license: "GPL",
    params: {
        my_bool: bool {
            default: true,
            description: "Example of bool",
        },
        my_i32: i32 {
            default: 42,
            description: "Example of i32",
        },
        my_usize: usize {
            default: 42,
            description: "Example of usize",
        },
    },
}

struct RustModuleParameters;

impl kernel::Module for RustModuleParameters {
    fn init(_module: &'static ThisModule) -> kernel::error::Result<Self> {
        pr_info!("Rust module parameters sample (init)\n");
        pr_info!("Parameters: \n");
        pr_info!("  my_bool: {}\n", module_parameters::my_bool.value());
        pr_info!("  my_i32: {}\n", module_parameters::my_i32.value());
        pr_info!("  my_usize: {}\n", module_parameters::my_usize.value());
        
        Ok(RustModuleParameters)
    }
}

impl Drop for RustModuleParameters {
    fn drop(&mut self) {
        pr_info!("Rust module parameters sample (exit)\n");
    }
}
