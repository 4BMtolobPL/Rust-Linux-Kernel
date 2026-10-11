// SPDC-License-Identifier: GPL-2.0

//! Rust memory alloc sample.

use core::alloc::Layout;

use kernel::{
    alloc::{allocator::Kmalloc, Allocator, NumaNode},
    prelude::*,
};

// 모듈 메타데이터 선언
module! {
    type: RustMemoryAlloc,
    name: "rust_momory_alloc",
    authors: ["Rust for Linux Contributors"],
    description: "Rust memory alloc sample",
    license: "GPL",
}

struct RustMemoryAlloc;

impl kernel::Module for RustMemoryAlloc {
    // 모듈이 초기화될 때 호출되는 함수
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust memory allocation sample (init)\n");

        pr_info!("Global allocators\n");

        {
            // `u16` 타입에 대한 메모리 레이아웃 생성
            let layout = Layout::new::<u16>();

            // 레이아웃에 따라 메모리 할당
            let ptr = Kmalloc::alloc(layout, GFP_KERNEL, NumaNode::NO_NODE)?;
            let ptr = ptr.cast::<u16>();

            // 할당된 메모리에 대한 참조 및 값 할당
            unsafe { ptr.write(42) };
            // 메모리 할당 및 할당이 예상대로 작동했는지 확인
            assert_eq!(unsafe { ptr.read() }, 42);

            // 메모리 해제
            unsafe { Kmalloc::free(ptr.cast(), layout) };
        }

        {
            let layout = Layout::new::<u16>();
            let ptr = Kmalloc::alloc(layout, GFP_KERNEL | __GFP_ZERO, NumaNode::NO_NODE)?;
            let ptr = ptr.cast::<u16>();

            // 메모리가 제로로 초기화됐는지 확인
            assert_eq!(unsafe { ptr.read() }, 0);

            // 메모리 해제
            unsafe { Kmalloc::free(ptr.cast(), layout) };
        }

        // 힙에 메모리 할당
        pr_info!("Heap allocation\n");
        let val: u32 = 5;

        let boxed: Box<u32, Kmalloc> = Box::new(val, GFP_KERNEL)?;
        pr_info!("boxed: {}\n", boxed);

        let boxed: Box<u32, Kmalloc> = Box::new(6, GFP_KERNEL)?;
        let val: u32 = *boxed;
        pr_info!("val: {}\n", val);

        pr_info!("Vector allocation\n");
        let mut v: Vec<i32, Kmalloc> = Vec::new();

        v.push(1, GFP_KERNEL)?;
        v.push(2, GFP_KERNEL)?;
        v.push(3, GFP_KERNEL)?;
        pr_info!("v[0]: {}, v[1]: {}, v[2]: {}\n", v[0], v[1], v[2]);

        // 초기화 성공
        Ok(RustMemoryAlloc)
    }
}

impl Drop for RustMemoryAlloc {
    // 모듈이 언로드될 때 호출되는 함수
    fn drop(&mut self) {
        pr_info!("Rust memory allocation sample (exit)\n");
    }
}
