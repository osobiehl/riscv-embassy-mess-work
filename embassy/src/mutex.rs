/// Async mutex.
///
/// The mutex is generic over a blocking [`RawMutex`](crate::blocking_mutex::raw::RawMutex).
/// The raw mutex is used to guard access to the internal "is locked" flag. It
/// is held for very short periods only, while locking and unlocking. It is *not* held
/// for the entire time the async Mutex is locked.
use core::cell::{RefCell, UnsafeCell};
use core::ops::{Deref, DerefMut};
use core::task::Poll;
use futures::future::poll_fn;

use crate::blocking_mutex::raw::RawMutex;
use crate::blocking_mutex::Mutex as BlockingMutex;
use crate::waitqueue::WakerRegistration;

/// Error returned by [`Mutex::try_lock`]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TryLockError;

struct State {
    locked: bool,
    waker: WakerRegistration,
}

pub struct Mutex<M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    state: BlockingMutex<M, RefCell<State>>,
    inner: UnsafeCell<T>,
}

unsafe impl<M: RawMutex + Send, T: ?Sized + Send> Send for Mutex<M, T> {}
unsafe impl<M: RawMutex + Sync, T: ?Sized + Send> Sync for Mutex<M, T> {}

/// Async mutex.
impl<M, T> Mutex<M, T>
where
    M: RawMutex,
{
    /// Create a new mutex with the given value.
    #[cfg(feature = "nightly")]
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            state: BlockingMutex::new(RefCell::new(State {
                locked: false,
                waker: WakerRegistration::new(),
            })),
        }
    }

    /// Create a new mutex with the given value.
    #[cfg(not(feature = "nightly"))]
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            state: BlockingMutex::new(RefCell::new(State {
                locked: false,
                waker: WakerRegistration::new(),
            })),
        }
    }
}

impl<M, T> Mutex<M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    /// Lock the mutex.
    ///
    /// This will wait for the mutex to be unlocked if it's already locked.
    pub async fn lock(&self) -> MutexGuard<'_, M, T> {
        poll_fn(|cx| {
            let ready = self.state.lock(|s| {
                let mut s = s.borrow_mut();
                if s.locked {
                    s.waker.register(cx.waker());
                    false
                } else {
                    s.locked = true;
                    true
                }
            });

            if ready {
                Poll::Ready(MutexGuard { mutex: self })
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Attempt to immediately lock the mutex.
    ///
    /// If the mutex is already locked, this will return an error instead of waiting.
    pub fn try_lock(&self) -> Result<MutexGuard<'_, M, T>, TryLockError> {
        self.state.lock(|s| {
            let mut s = s.borrow_mut();
            if s.locked {
                Err(TryLockError)
            } else {
                s.locked = true;
                Ok(())
            }
        })?;

        Ok(MutexGuard { mutex: self })
    }
}

/// Async mutex guard.
///
/// Owning an instance of this type indicates having
/// successfully locked the mutex, and grants access to the contents.
///
/// Dropping it unlocks the mutex.
pub struct MutexGuard<'a, M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    mutex: &'a Mutex<M, T>,
}

impl<'a, M, T> Drop for MutexGuard<'a, M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    fn drop(&mut self) {
        self.mutex.state.lock(|s| {
            let mut s = s.borrow_mut();
            s.locked = false;
            s.waker.wake();
        })
    }
}

impl<'a, M, T> Deref for MutexGuard<'a, M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: the MutexGuard represents exclusive access to the contents
        // of the mutex, so it's OK to get it.
        unsafe { &*(self.mutex.inner.get() as *const T) }
    }
}

impl<'a, M, T> DerefMut for MutexGuard<'a, M, T>
where
    M: RawMutex,
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the MutexGuard represents exclusive access to the contents
        // of the mutex, so it's OK to get it.
        unsafe { &mut *(self.mutex.inner.get()) }
    }
}
