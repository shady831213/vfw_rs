use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct MsgCell<T> {
    next_ticket: AtomicUsize,
    next_serving: AtomicUsize,
    val: UnsafeCell<T>,
}

impl<T> MsgCell<T> {
    pub const fn new(data: T) -> Self {
        Self {
            next_ticket: AtomicUsize::new(0),
            next_serving: AtomicUsize::new(0),
            val: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub unsafe fn local(&self) -> LocalMsgCell<'_, T> {
        LocalMsgCell(self)
    }

    #[inline(always)]
    pub fn remote(&self) -> RemoteMsgCell<'_, T> {
        RemoteMsgCell(self)
    }
}

pub struct RemoteMsgCell<'a, T>(&'a MsgCell<T>);

impl<T> RemoteMsgCell<'_, T> {
    #[inline(always)]
    pub fn send(self, t: T) {
        let ticket = self.0.next_ticket.fetch_add(1, Ordering::Relaxed);
        while self.0.next_serving.load(Ordering::Acquire) != ticket {
            spin_loop();
        }
        unsafe { *self.0.val.get() = t };
    }
}

pub struct LocalMsgCell<'a, T>(&'a MsgCell<T>);
impl<'a, T> LocalMsgCell<'a, T> {
    #[inline(always)]
    pub fn recv(&self) -> &'a T {
        unsafe { &*self.0.val.get() }
    }

    #[inline(always)]
    pub fn done(&self) {
        self.0.next_serving.fetch_add(1, Ordering::Release);
    }
}
