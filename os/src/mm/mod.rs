//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

use crate::task::TASK_MANAGER;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// convert virt address to phy address
pub fn translate<T>(token: usize, ptr: *mut T) -> &'static mut T {
    let from_token = PageTable::from_token(token);
    let from = VirtAddr::from(ptr as usize);
    let actual_address: PhysAddr = from_token
        .find_pte(from.clone().floor())
        .map(|entry| {
            let pta: PhysAddr = entry.ppn().into();
            (pta.0 + from.page_offset()).into()
        })
        .unwrap();
    actual_address.get_mut()
}

/// Just look at this name...
pub fn alloc_virtual_memory(start: usize, len: usize, port: usize) -> isize {
    let from = VirtAddr::from(start);
    let mut to = VirtAddr::from(start + len);

    let from_vpn = from.floor();
    let to_vpn = to.ceil();

    to = VirtAddr::from(to_vpn);

    TASK_MANAGER.operate_memset(|memory_set| unsafe {
        if (*memory_set).page_table.interval_valid(from_vpn, to_vpn) {
            return -1;
        }
        (*memory_set).insert_framed_area(
            from.into(),
            to.into(),
            MapPermission::from_bits((port << 1 | 16) as u8).unwrap(),
        );
        return 0;
    })
}

/// free virtual memory
pub fn free_virtual_memory(start: usize, len: usize) -> isize {
    let from = VirtAddr::from(start);
    let to = VirtAddr::from(start + len);

    let from_vpn = from.floor();
    let to_vpn = to.ceil();

    TASK_MANAGER.operate_memset(|memory_set| unsafe {
        if (*memory_set).page_table.interval_invalid(from_vpn, to_vpn) {
            return -1;
        }
        let mut res: isize = -1;

        (*memory_set)
            .page_table
            .interval_op(from_vpn, to_vpn, |num| {
                if let Some(val) = (*memory_set)
                    .areas
                    .iter_mut()
                    .find(|area| area.vpn_range.get_start() == num)
                {
                    val.unmap(&mut (*memory_set).page_table);
                    res = 0;
                }
            });
        return res;
    })
}
