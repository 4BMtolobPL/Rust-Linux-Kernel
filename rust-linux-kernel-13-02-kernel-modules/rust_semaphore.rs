// SPDX-License-Identifier: GPL-2.0

//! 러스트 세마포어 예제
//! 사용자 공간에서 사용할 수 있는 카운팅 세마포어

use core::{sync::atomic::AtomicU64};

use kernel::{
    fs::{File, Kiocb}, global_lock, ioctl::{_IOC_SIZE, _IOW, _IOR}, iov::{IovIterDest, IovIterSource}, miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration}, new_condvar, new_mutex, prelude::*, sync::{Arc, CondVar, Mutex}, uaccess::UserSlice,
};

const IOCTL_GET_READ_COUNT: u32 = _IOR::<u64>('c' as u32, 1);
const IOCTL_SET_READ_COUNT: u32 = _IOW::<u64>('c' as u32, 1);

module! {
    type: RustSemaphore,
    name: "rust_semaphore",
    authors: ["Rust for Linux Contributors"],
    description: "Rust semaphore sample",
    license: "GPL",
}

// ----------------------------------------------------------------------------
// Shared semaphore
// ----------------------------------------------------------------------------

global_lock! {
    unsafe(uninit) static GLOBAL_SEMAPHORE: Mutex<Option<Arc<Semaphore>>> = None;
}

// ----------------------------------------------------------------------------
// Semaphore state
// ----------------------------------------------------------------------------

struct SemaphoreInner {
    count: usize,
    max_seen: usize,
}

#[pin_data]
struct Semaphore {
    #[pin]
    changed: CondVar,
    #[pin]
    inner: Mutex<SemaphoreInner>,
}

impl Semaphore {
    fn consume(&self) -> Result {
        let mut inner = self.inner.lock();

        while inner.count == 0 {
            if self.changed.wait_interruptible(&mut inner) {
                return Err(EINTR);
            }
        }

        inner.count -=1;
        Ok(())
    }

    fn release(&self, count: usize) {
        {
            let mut inner = self.inner.lock();

            inner.count = inner.count.saturating_add(count);
    
            if inner.count > inner.max_seen {
                inner.max_seen = inner.count;
            }
        }

        self.changed.notify_all();
    }
}

// ----------------------------------------------------------------------------
// Per-open file state
// ----------------------------------------------------------------------------

struct FileState {
    read_count: AtomicU64,
    shared: Arc<Semaphore>,
}

// impl FileState {
//     fn consume(&self) -> Result {
//         let mut inner = self.shared.inner.lock();
//         while inner.count == 0 {
//             if self.shared.changed.wait_interruptible(&mut inner) {
//                 return Err(EINTR);
//             }
//         }
//         inner.count -= 1;
//         Ok(())
//     }
// }

#[vtable]
impl MiscDevice for FileState {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, _misc: &MiscDeviceRegistration<Self>) -> Result<Self::Ptr> {
        let shared = {
            let guard = GLOBAL_SEMAPHORE.lock();

            guard.as_ref().ok_or(ENODEV)?.clone()
        };
        
        KBox::pin_init(
            FileState {
                read_count: AtomicU64::new(0),
                shared: shared
            },
            GFP_KERNEL,
        )
    }

    fn read_iter(mut kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterDest<'_>) -> Result<usize> {
        if iov.is_empty() || kiocb.ki_pos() > 0 {
            return Ok(0);
        }

        let me = kiocb.file();
        me.shared.consume()?;
        iov.copy_to_iter(&[0u8; 1]);
        me.read_count.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        *kiocb.ki_pos_mut() += 1;
        
        Ok(1)
    }

    /// Write to this miscdevice.
    fn write_iter(kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterSource<'_>) -> Result<usize> {
        let me = kiocb.file();

        let count = iov.len();
        if count == 0 {
            return Ok(0);
        }

        me.shared.release(count);

        iov.advance(count);
        Ok(count)
    }

    fn ioctl(me: Pin<&FileState>, _file: &File, cmd: u32, arg: usize) -> Result<isize> {
        let arg = UserPtr::from_addr(arg);
        let size = _IOC_SIZE(cmd);

        match cmd {
            IOCTL_GET_READ_COUNT => {
                let value = me.read_count.load(core::sync::atomic::Ordering::Relaxed);
                UserSlice::new(arg, size).writer().write(&value)?;
            },
            IOCTL_SET_READ_COUNT => {
                let value: u64 = UserSlice::new(arg, size).reader().read()?;
                me.read_count.store(value, core::sync::atomic::Ordering::Relaxed);
            },
            _ => {
                return Err(ENOTTY);
            }
        }

        Ok(0)
    }
}

struct RustSemaphore {
    _miscdev: Pin<KBox<MiscDeviceRegistration<FileState>>>,
}

impl kernel::Module for RustSemaphore {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust semaphore sample (init)\n");

        unsafe { GLOBAL_SEMAPHORE.init() };

        let semaphore = Arc::pin_init(try_pin_init!{
            Semaphore {
                changed <- new_condvar!(),
                inner <- new_mutex!(SemaphoreInner {
                    count: 0,
                    max_seen: 0,
                }),
            }
        }, GFP_KERNEL)?;

        let options = MiscDeviceOptions {
            name: c"rust_semaphore",
        };

        let miscdev = KBox::pin_init(MiscDeviceRegistration::register(options), GFP_KERNEL)?;
        
        {
            let mut guard = GLOBAL_SEMAPHORE.lock();
            *guard = Some(semaphore);
        }

        Ok(Self {
            _miscdev: miscdev,
        })
    }
}

impl Drop for RustSemaphore {
    fn drop(&mut self) {
        pr_info!("Rust semaphore sample (exit)\n");
        {
            let mut guard = GLOBAL_SEMAPHORE.lock();
            *guard = None;
        }
    }
}
