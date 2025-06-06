use core::{cell::UnsafeCell, hint::spin_loop};

pub struct LocalHsmCell<'a, T>(&'a HsmCell<T>);

pub struct RemoteHsmCell<'a, T>(&'a HsmCell<T>);

unsafe impl<T: Send> Sync for HsmCell<T> {}
unsafe impl<T: Send> Send for HsmCell<T> {}

// avoid to depend arch
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(usize)]
pub enum HsmState {
    Stopped = 0,
    Started,
    StartPending,
    StopPending,
    Suspended,
    SuspendPending,
    ResumePending,
    StartPendingExt = usize::MAX,
}

impl core::convert::TryFrom<usize> for HsmState {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HsmState::Stopped),
            1 => Ok(HsmState::Started),
            2 => Ok(HsmState::StartPending),
            3 => Ok(HsmState::StopPending),
            4 => Ok(HsmState::Suspended),
            5 => Ok(HsmState::SuspendPending),
            6 => Ok(HsmState::ResumePending),
            usize::MAX => Ok(HsmState::StartPendingExt),
            v => Err(v),
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
mod hsm_imp {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};
    pub struct HsmCell<T> {
        status: AtomicUsize,
        val: UnsafeCell<Option<T>>,
    }

    impl<T> HsmCell<T> {
        pub const fn new() -> Self {
            Self {
                status: AtomicUsize::new(HsmState::Stopped as usize),
                val: UnsafeCell::new(None),
            }
        }
    }

    impl<T> LocalHsmCell<'_, T> {
        #[inline(always)]
        pub fn start(&self) -> Result<T, HsmState> {
            loop {
                match self.0.status.compare_exchange(
                    HsmState::StartPending as usize,
                    HsmState::Started as usize,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break Ok(unsafe { (*self.0.val.get()).take().unwrap() }),
                    Err(s) => {
                        if s == HsmState::StartPendingExt as usize {
                            spin_loop()
                        } else {
                            let state = HsmState::try_from(s).expect("Illegle hsm state!");
                            break Err(state);
                        }
                    }
                }
            }
        }

        #[inline(always)]
        pub fn stop(&self) {
            self.0
                .status
                .store(HsmState::Stopped as usize, Ordering::Release)
        }

        #[inline(always)]
        pub fn suspend(&self) {
            self.0
                .status
                .store(HsmState::Suspended as usize, Ordering::Relaxed)
        }

        #[inline(always)]
        pub fn resume(&self) {
            self.0
                .status
                .store(HsmState::Started as usize, Ordering::Relaxed)
        }
    }

    impl<T> RemoteHsmCell<'_, T> {
        #[inline(always)]
        pub fn start(self, t: T) -> bool {
            if self
                .0
                .status
                .compare_exchange(
                    HsmState::Stopped as usize,
                    HsmState::StartPendingExt as usize,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                unsafe { *self.0.val.get() = Some(t) };
                self.0
                    .status
                    .store(HsmState::StartPending as usize, Ordering::Release);
                true
            } else {
                false
            }
        }

        #[inline(always)]
        pub fn get_status(&self) -> Result<HsmState, usize> {
            HsmState::try_from(self.0.status.load(Ordering::Relaxed))
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
mod hsm_no_atomic {
    use super::*;
    pub struct HsmCell<T> {
        status: UnsafeCell<usize>,
        val: UnsafeCell<Option<T>>,
    }

    impl<T> HsmCell<T> {
        pub const fn new() -> Self {
            Self {
                status: UnsafeCell::new(HsmState::Stopped as usize),
                val: UnsafeCell::new(None),
            }
        }
    }

    impl<T> LocalHsmCell<'_, T> {
        #[inline(always)]
        pub fn start(&self) -> Result<T, HsmState> {
            loop {
                let current = unsafe { *self.0.status.get() };
                if current == HsmState::StartPending as usize {
                    unsafe { *self.0.status.get() = HsmState::Started as usize };
                    break Ok(unsafe { (*self.0.val.get()).take().unwrap() });
                } else if current == HsmState::StartPendingExt as usize {
                    spin_loop()
                } else {
                    let state = HsmState::try_from(current).expect("Illegle hsm state!");
                    break Err(state);
                }
            }
        }

        #[inline(always)]
        pub fn stop(&self) {
            unsafe { *self.0.status.get() = HsmState::Stopped as usize };
        }

        #[inline(always)]
        pub fn suspend(&self) {
            unsafe { *self.0.status.get() = HsmState::Suspended as usize };
        }

        #[inline(always)]
        pub fn resume(&self) {
            unsafe { *self.0.status.get() = HsmState::Started as usize };
        }
    }

    impl<T> RemoteHsmCell<'_, T> {
        #[inline(always)]
        pub fn start(self, t: T) -> bool {
            let current = unsafe { *self.0.status.get() };
            if current == HsmState::Stopped as usize {
                unsafe { *self.0.status.get() = HsmState::StartPendingExt as usize };
                unsafe { *self.0.val.get() = Some(t) };
                unsafe { *self.0.status.get() = HsmState::StartPending as usize };
                true
            } else {
                false
            }
        }

        #[inline(always)]
        pub fn get_status(&self) -> Result<HsmState, usize> {
            HsmState::try_from(unsafe { *self.0.status.get() })
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
pub use hsm_imp::*;

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub use hsm_no_atomic::*;

impl<T> HsmCell<T> {
    #[inline(always)]
    pub unsafe fn local(&self) -> LocalHsmCell<'_, T> {
        LocalHsmCell(self)
    }

    #[inline(always)]
    pub fn remote(&self) -> RemoteHsmCell<'_, T> {
        RemoteHsmCell(self)
    }
}
