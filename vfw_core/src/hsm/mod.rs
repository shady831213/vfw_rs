use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct HsmCell<T> {
    status: AtomicUsize,
    val: UnsafeCell<Option<T>>,
}

pub struct LocalHsmCell<'a, T>(&'a HsmCell<T>);

pub struct RemoteHsmCell<'a, T>(&'a HsmCell<T>);

unsafe impl<T: Send> Sync for HsmCell<T> {}
unsafe impl<T: Send> Send for HsmCell<T> {}

// avoid to depend arch
#[repr(usize)]
pub enum HsmState {
    Started = 0,
    Stopped,
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
            0 => Ok(HsmState::Started),
            1 => Ok(HsmState::Stopped),
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

impl<T> HsmCell<T> {
    pub const fn new() -> Self {
        Self {
            status: AtomicUsize::new(HsmState::Stopped as usize),
            val: UnsafeCell::new(None),
        }
    }

    #[inline]
    pub unsafe fn local(&self) -> LocalHsmCell<'_, T> {
        LocalHsmCell(self)
    }

    #[inline]
    pub fn remote(&self) -> RemoteHsmCell<'_, T> {
        RemoteHsmCell(self)
    }
}

impl<T> LocalHsmCell<'_, T> {
    #[inline]
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
                        // TODO: handle illegel state
                        break Err(HsmState::try_from(s).unwrap());
                    }
                }
            }
        }
    }

    #[inline]
    pub fn stop(&self) {
        self.0
            .status
            .store(HsmState::Stopped as usize, Ordering::Release)
    }

    #[inline]
    pub fn suspend(&self) {
        self.0
            .status
            .store(HsmState::Suspended as usize, Ordering::Relaxed)
    }

    #[inline]
    pub fn resume(&self) {
        self.0
            .status
            .store(HsmState::Started as usize, Ordering::Relaxed)
    }
}

impl<T> RemoteHsmCell<'_, T> {
    #[inline]
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

    #[inline]
    pub fn get_status(&self) -> Result<HsmState, usize> {
        HsmState::try_from(self.0.status.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn allow_ipi(&self) -> bool {
        let s = self.0.status.load(Ordering::Relaxed);
        s == HsmState::Started as usize || s == HsmState::Suspended as usize
    }
}
