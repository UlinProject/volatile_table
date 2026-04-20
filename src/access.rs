use core::fmt::Debug;

use cluFullTransmute::transmute_unchecked;

pub trait VolatilePtrAccess<T>: Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord {
    type TPtr: Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord;

    fn from_usize(ptr: usize) -> Self::TPtr;
    fn into_usize(self) -> usize;
}

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

pub(crate) const unsafe fn into_usize_unchecked<A: VolatilePtrAccess<T>, T>(v: A::TPtr) -> usize {
    unsafe { transmute_unchecked(v) }
}
