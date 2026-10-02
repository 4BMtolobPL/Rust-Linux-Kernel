// SPDX-License-Identifier: GPL-2.0

//! Rust synchronisation primitives sample.

use kernel::{new_condvar, new_mutex, new_spinlock, prelude::*, sync::{CondVar, Mutex, SpinLock}};

module! {
    type: RustSync,
    name: "rust_sync",
    authors: ["Rust for Linux Contributors"],
    description: "Rust synchronisation primitives sample",
    license: "GPL",
}

struct RustSync;

#[pin_data]
struct MutexExample {
    #[pin]
    value: Mutex<u32>,
    #[pin]
    value_changed: CondVar,
}

#[pin_data]
struct SpinLockExample {
    #[pin]
    value: SpinLock<u32>,
    #[pin]
    value_changed: CondVar,
}

impl kernel::Module for RustSync {
    fn init(_module: &'static ThisModule) -> kernel::error::Result<Self> {
        pr_info!("Rust synchronisation primitives sample (init)\n");

        // mutex 테스트
        {
            let mutex_example = KBox::pin_init(pin_init!(MutexExample {
                value <- new_mutex!(0),
                value_changed <- new_condvar!(),
            }), GFP_KERNEL)?;

            *mutex_example.value.lock() = 10;
            pr_info!("Value: {}\n", *mutex_example.value.lock());
            
            {
                let mut guard = mutex_example.value.lock();
                while *guard != 10 {
                    mutex_example.value_changed.wait(&mut guard);
                }
            }
            mutex_example.value_changed.notify_one();
            mutex_example.value_changed.notify_all();
        }

        // SpinLock 테스트
        {
            let spinlock_example = KBox::pin_init(pin_init!(SpinLockExample {
                value <- new_spinlock!(0),
                value_changed <- new_condvar!(),
            } ), GFP_KERNEL)?;

            *spinlock_example.value.lock() = 10;
            pr_info!("Value: {}\n", *spinlock_example.value.lock());

            {
                let mut guard = spinlock_example.value.lock();
                while *guard != 10 {
                    spinlock_example.value_changed.wait(&mut guard);
                }
            }

            spinlock_example.value_changed.notify_one();
            spinlock_example.value_changed.notify_all();
        }
        
        Ok(RustSync)
    }
}

impl Drop for RustSync {
    fn drop(&mut self) {
        pr_info!("Rust synchronisation primitives sample (exit)\n");
    }
}
