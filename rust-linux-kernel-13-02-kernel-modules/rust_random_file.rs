//! Adapted from Alex Gaynor's original available at
//! <https://github.com/alex/just-use/blob/master/src/lib.rs>.

use kernel::{
    fs::{File, Kiocb}, iov::{IovIterDest, IovIterSource}, miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration}, new_mutex, prelude::*, sync::Mutex,
};

unsafe extern "C" {
    fn get_random_bytes(buf: *mut kernel::ffi::c_void, len: usize);
    fn add_device_randomness(buf: *const kernel::ffi::c_void, len: usize);
}

module! {
    type: RandomFileModule,
    name: "rust_random",
    authors: ["Rust for Linux Contributors"],
    description: "Just use /dev/urandom: Now with early-boot safety",
    license: "GPL",
}

#[pin_data]
struct RandomFileModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<RandomFile>,
}

impl kernel::InPlaceModule for RandomFileModule {
    fn init(_module: &'static ThisModule) -> impl pin_init::PinInit<Self, kernel::error::Error> {
        pr_info!("Initializing Rust random file sample\n");

        let options = MiscDeviceOptions {
            name: c"rust-random-file",
        };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options),
        })
    }
}

#[pin_data(PinnedDrop)]
struct RandomFile {
    #[pin]
    buffer: Mutex<KVec<u8>>, // TODO: Vec, KVec, VVec, KVVec의 차이
}

#[vtable]
impl MiscDevice for RandomFile {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, _misc: &MiscDeviceRegistration<Self>) -> Result<Self::Ptr> {
        pr_info!("Opening Rust random file sample\n");

        KBox::pin_init(
            pin_init!(RandomFile {
                buffer <- new_mutex!(KVec::new()),
            }),
            GFP_KERNEL,
        )
    }

    fn read_iter(mut kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterDest<'_>) -> Result<usize> {
        pr_info!("Reading from Rust random file sample\n");

        let mut total_len = 0;
        let mut chuck_buf = [0; 256];

        while !iov.is_empty() {
            let len = iov.len().min(chuck_buf.len());
            let chunk = &mut chuck_buf[0..len];

            // TODO: blocking 관련 코드 구현 안했음
            unsafe {
                get_random_bytes(chunk.as_mut_ptr().cast::<kernel::ffi::c_void>(), len);
            }

            total_len += iov.simple_read_from_buffer(kiocb.ki_pos_mut(), chunk.as_bytes())?;
        }

        Ok(total_len)
    }

    fn write_iter(mut _kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterSource<'_>) -> Result<usize> {
        pr_info!("Writing to Rust random file sample\n");

        let mut total_len = 0;
        let mut chunk_buf = [0; 256];

        while !iov.is_empty() {
            let len = iov.len().min(chunk_buf.len());
            let chuck = &mut chunk_buf[0..len];

            total_len += iov.copy_from_iter(chuck.as_mut_bytes());
            unsafe {
                add_device_randomness(chuck.as_bytes().as_ptr() as *const core::ffi::c_void, chuck.len());
            }
        }

        Ok(total_len)
    }
}

#[pinned_drop]
impl PinnedDrop for RandomFile {
    fn drop(self: Pin<&mut Self>) {
        pr_info!("Exiting the Rust random file sample\n");
    }
}
