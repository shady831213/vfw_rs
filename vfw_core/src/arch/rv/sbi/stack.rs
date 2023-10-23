use crate::{arch::FlowContext, hartid, HsmCell, LocalHsmCell, RemoteHsmCell, Stack, TrapHandler};
use core::ptr::NonNull;
#[derive(Debug)]
pub struct Supervisor {
    pub start_addr: usize,
    pub opaque: usize,
}

pub struct SbiHartContext {
    trap: FlowContext,
    pub hsm: HsmCell<Supervisor>,
}

impl SbiHartContext {
    #[inline]
    pub fn init(&mut self) {
        self.hsm = HsmCell::new();
    }

    #[inline]
    pub fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

pub struct SbiStacks<const SIZE: usize, const NUM: usize>([SbiStack<SIZE>; NUM]);

impl<const SIZE: usize, const NUM: usize> SbiStacks<SIZE, NUM> {
    pub const ZERO: Self = Self([SbiStack::<SIZE>::ZERO; NUM]);
    pub fn prepare_for_trap(
        &mut self,
        handler: TrapHandler,
        hart_id: usize,
        start_addr: usize,
        opaque: usize,
    ) {
        unsafe {
            self.get_unchecked_mut(hartid()).load_as_stack(handler);
        }
        if hartid() == hart_id {
            self.local_remote_hsm()
                .start(Supervisor { start_addr, opaque });
        }
    }

    pub fn local_hsm(&mut self) -> LocalHsmCell<'_, Supervisor> {
        unsafe { self.get_unchecked_mut(hartid()).hart_context().hsm.local() }
    }

    pub fn local_remote_hsm(&mut self) -> RemoteHsmCell<'_, Supervisor> {
        unsafe { self.get_unchecked_mut(hartid()).hart_context().hsm.remote() }
    }

    pub fn remote_hsm(&mut self, hart_id: usize) -> Option<RemoteHsmCell<'_, Supervisor>> {
        self.get_mut(hart_id).map(|x| x.hart_context().hsm.remote())
    }
}

impl<const SIZE: usize, const NUM: usize> core::ops::Deref for SbiStacks<SIZE, NUM> {
    type Target = [SbiStack<SIZE>; NUM];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const SIZE: usize, const NUM: usize> core::ops::DerefMut for SbiStacks<SIZE, NUM> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(C)]
pub struct SbiStack<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> SbiStack<SIZE> {
    pub const ZERO: Self = Self([0; SIZE]);
    #[inline]
    pub fn hart_context(&mut self) -> &mut SbiHartContext {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }
    #[inline]
    pub fn load_as_stack(&mut self, handler: TrapHandler) {
        let hart = self.hart_context();
        let context_ptr = hart.context_ptr();
        hart.init();
        self.load_context(context_ptr, handler);
    }
}

impl<const SIZE: usize> Stack for SbiStack<SIZE> {
    #[inline]
    fn start(&self) -> usize {
        self.0.as_ptr() as usize + SIZE
    }
    #[inline]
    fn size(&self) -> usize {
        SIZE
    }
}

#[macro_export]
macro_rules! stack_locate {
    ($sbi_stack:expr, $size:expr) => {
        #[naked]
        unsafe extern "C" fn stack_locate() {
            core::arch::asm!(
                "   la   sp, {stack}
                    li   t0, {per_hart_stack_size}
                    csrr t1, mhartid
                    addi t1, t1,  1
                 1: add  sp, sp, t0
                    addi t1, t1, -1
                    bnez t1, 1b
                    call t1, {move_stack}
                    ret
                ",
                per_hart_stack_size = const $size,
                stack               =   sym $sbi_stack,
                move_stack          =   sym crate::reuse_stack_for_trap,
                options(noreturn),
            )
        }
    }
}
