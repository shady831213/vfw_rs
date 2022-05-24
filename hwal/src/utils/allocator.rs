extern crate alloc;
use alloc::collections::linked_list::LinkedList;
use core::alloc::Layout;

// #[derive(Copy, Clone, Debug)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    pub start: usize,
    pub size: usize,
}

fn align_down(addr: usize, align: usize) -> usize {
    if align == 0 {
        return addr;
    }
    assert!(align.is_power_of_two());
    addr & !(align - 1)
}

fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

pub struct Allocator {
    free_blocks: LinkedList<Block>,
}

impl Allocator {
    pub fn init(&mut self) {
        *self = Self::empty();
    }
    pub const fn empty() -> Self {
        Allocator {
            free_blocks: LinkedList::new(),
        }
    }
    pub fn add_block(&mut self, start: usize, size: usize) {
        self.free_blocks.push_front(Block { start, size })
    }

    fn delete_block<F: Fn(&Block) -> bool>(&mut self, f: F) -> Option<Block> {
        let mut found: Option<usize> = None;
        for (i, e) in self.free_blocks.iter().enumerate() {
            if f(e) {
                found = Some(i);
                break;
            }
        }
        found.map(|i| self.free_blocks.remove(i))
    }

    pub fn alloc(&mut self, layout: Layout) -> Option<Block> {
        let (size, align) = (layout.size(), layout.align());
        let block = self
            .delete_block(|info| info.size >= size + (align_up(info.start, align) - info.start));
        if let Some(info) = block {
            let result = Block {
                start: align_up(info.start, align),
                size: size,
            };
            if align_up(info.start, align) != info.start {
                self.free_blocks.push_front(Block {
                    start: info.start,
                    size: align_up(info.start, align) - info.start,
                });
            }
            if info.size != size + (align_up(info.start, align) - info.start) {
                self.free_blocks.push_front(Block {
                    start: align_up(info.start, align) + size,
                    size: info.size - size - (align_up(info.start, align) - info.start),
                });
            }
            // crate::println!("after alloc {:x?}", self.free_blocks);
            Some(result)
        } else {
            None
        }
    }

    pub fn free(&mut self, addr: usize, layout: Layout) {
        let pre_block = self.delete_block(|info| info.start + info.size == addr);
        let pre_info = if let Some(info) = pre_block {
            Block {
                start: info.start,
                size: info.size + layout.size(),
            }
        } else {
            Block {
                start: addr,
                size: layout.size(),
            }
        };

        let post_block = self.delete_block(|info| pre_info.start + pre_info.size == info.start);
        let post_info = if let Some(info) = post_block {
            Block {
                start: pre_info.start,
                size: pre_info.size + info.size,
            }
        } else {
            pre_info
        };
        self.free_blocks.push_front(post_info);
        // crate::println!("free addr {:x} size {:x}", addr, layout.size());
        // crate::println!("after free {:x?}", self.free_blocks);
    }
}

#[no_mangle]
extern "C" fn allocator_init(a: &mut Allocator) {
    a.init()
}

#[no_mangle]
extern "C" fn allocator_add_block(a: &mut Allocator, start: usize, size: usize) {
    a.add_block(start, size)
}

#[no_mangle]
extern "C" fn allocator_alloc(a: &mut Allocator, size: usize, align: usize) -> *mut u8 {
    if let Some(b) = a.alloc(Layout::from_size_align(size, align).unwrap()) {
        b.start as *mut u8
    } else {
        crate::println!("oom!");
        panic!()
    }
}

#[no_mangle]
extern "C" fn allocator_free(a: &mut Allocator, start: usize, size: usize) {
    a.free(start, Layout::from_size_align(size, 1).unwrap())
}
