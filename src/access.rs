//! Access control markers for volatile pointers.
//!
//! This module defines markers ([`RO`], [`RW`], [`WO`]) used to specify
//! the capabilities of a [`VolatilePtr`]. These markers are zero-sized
//! and exist only to enforce access rules at compile time.

use cluFullTransmute::transmute_unchecked;
use core::fmt::Debug;

/// A trait that defines how a specific access marker relates to raw pointers.
pub trait VolatilePtrAccess<T>: Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord {
    /// The underlying raw pointer type for this access level.
    type TPtr: Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord;

    /// Creates a raw pointer from a memory address.
    fn from_usize(ptr: usize) -> Self::TPtr;
    /// Converts the access marker instance to a usize address
    fn into_usize(self) -> usize;
}

/// **Read-Only** access marker.
///
/// Pointers with this marker only allow `.read()` operations.
/// Maps to `*const T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RO {}

impl<T> VolatilePtrAccess<T> for RO {
    type TPtr = *const T;

    #[inline]
    fn from_usize(ptr: usize) -> Self::TPtr {
        ptr as _
    }

    #[inline]
    fn into_usize(self) -> usize {
        self as _
    }
}

/// **Read-Write** access marker.
///
/// Pointers with this marker allow both `.read()` and `.write()` operations.
/// Maps to `*mut T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RW {}

impl<T> VolatilePtrAccess<T> for RW {
    type TPtr = *mut T;

    #[inline]
    fn from_usize(ptr: usize) -> Self::TPtr {
        ptr as _
    }

    #[inline]
    fn into_usize(self) -> usize {
        self as _
    }
}

/// **Write-Only** access marker.
///
/// Pointers with this marker only allow `.write()` operations.
/// Maps to `*mut T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WO {}

impl<T> VolatilePtrAccess<T> for WO {
    type TPtr = *mut T;

    #[inline]
    fn from_usize(ptr: usize) -> Self::TPtr {
        ptr as _
    }

    #[inline]
    fn into_usize(self) -> usize {
        self as _
    }
}

/// Forcefully converts a raw pointer type to its address representation.
///
/// # Safety
/// This is used internally to perform address arithmetic. It relies on the fact
/// that `*const T` and `*mut T` have the same layout as `usize`.
pub(crate) const unsafe fn into_usize_unchecked<A: VolatilePtrAccess<T>, T>(v: A::TPtr) -> usize {
    unsafe { transmute_unchecked(v) }
}
