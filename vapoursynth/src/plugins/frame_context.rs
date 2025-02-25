use std::marker::PhantomData;
use std::ptr::NonNull;
use vapoursynth_sys as ffi;

use crate::api::API;

/// A frame context used in filters.
#[derive(Debug, Clone, Copy)]
pub struct FrameContext<'a> {
    handle: NonNull<ffi::VSFrameContext>,
    _owner: PhantomData<&'a ()>,
}

impl FrameContext<'_> {
    /// Wraps `handle` in a `FrameContext`.
    ///
    /// # Safety
    /// The caller must ensure `handle` is valid and API is cached.
    #[inline]
    pub(crate) unsafe fn from_ptr(handle: *mut ffi::VSFrameContext) -> Self {
        unsafe {
            Self {
                handle: NonNull::new_unchecked(handle),
                _owner: PhantomData,
            }
        }
    }

    /// Returns the underlying pointer.
    #[inline]
    pub(crate) fn ptr(self) -> *mut ffi::VSFrameContext {
        self.handle.as_ptr()
    }

    /// Returns the index of the node from which the frame is being requested.
    #[inline]
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn output_index(self) -> usize {
        let index = unsafe { API::get_cached().get_output_index(self.handle.as_ptr()) };
        debug_assert!(index >= 0);
        index as _
    }

    /// Returns the index of the node from which the frame is being requested.
    #[inline]
    #[cfg(feature = "gte-vapoursynth-api-40")]
    pub fn output_index(self) -> usize {
        todo!()
    }
}
