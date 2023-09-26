use core::{cell::UnsafeCell, hint::spin_loop};

pub struct RemoteMsgCell<'a, T>(&'a MsgCell<T>);

pub struct LocalMsgCell<'a, T>(&'a MsgCell<T>);

#[cfg(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
))]
mod msg_imp {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    pub struct MsgCell<T> {
        next_ticket: AtomicUsize,
        next_serving: AtomicUsize,
        pub(super) val: UnsafeCell<T>,
    }

    impl<T> MsgCell<T> {
        pub const fn new(data: T) -> Self {
            Self {
                next_ticket: AtomicUsize::new(0),
                next_serving: AtomicUsize::new(0),
                val: UnsafeCell::new(data),
            }
        }
    }

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

    impl<'a, T> LocalMsgCell<'a, T> {
        #[inline(always)]
        pub fn done(&self) {
            self.0.next_serving.fetch_add(1, Ordering::Release);
        }
    }
}

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
mod msg_no_atomic {
    use super::*;
    pub struct MsgCell<T> {
        next_ticket: UnsafeCell<usize>,
        next_serving: UnsafeCell<usize>,
        pub(super) val: UnsafeCell<T>,
    }

    impl<T> MsgCell<T> {
        pub const fn new(data: T) -> Self {
            Self {
                next_ticket: UnsafeCell::new(0),
                next_serving: UnsafeCell::new(0),
                val: UnsafeCell::new(data),
            }
        }
    }

    impl<T> RemoteMsgCell<'_, T> {
        #[inline(always)]
        pub fn send(self, t: T) {
            unsafe {
                let ticket = *self.0.next_ticket.get();
                *self.0.next_ticket.get() = ticket + 1;
                while *self.0.next_serving.get() != ticket {
                    spin_loop();
                }
                *self.0.val.get() = t;
            }
        }
    }

    impl<'a, T> LocalMsgCell<'a, T> {
        #[inline(always)]
        pub fn done(&self) {
            unsafe {
                let next_ticket = *self.0.next_serving.get();
                *self.0.next_serving.get() = next_ticket + 1;
            }
        }
    }
}

#[cfg(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
))]
pub use msg_imp::*;

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub use msg_no_atomic::*;

impl<T> MsgCell<T> {
    #[inline(always)]
    pub unsafe fn local(&self) -> LocalMsgCell<'_, T> {
        LocalMsgCell(self)
    }

    #[inline(always)]
    pub fn remote(&self) -> RemoteMsgCell<'_, T> {
        RemoteMsgCell(self)
    }
}

impl<'a, T> LocalMsgCell<'a, T> {
    #[inline(always)]
    pub fn recv(&self) -> &'a T {
        unsafe { &*self.0.val.get() }
    }
}
