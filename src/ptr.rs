//! Type-safe volatile pointer implementation.
//!
//! [`VolatilePtr`] is a transparent wrapper around a raw pointer that uses 
//! type-state patterns to enforce access rights at compile time.

use crate::access::{RO, RW, VolatilePtrAccess, WO};
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ptr;

/// A volatile pointer with compile-time access control.
///
/// - `A`: Access marker ([`RO`], [`RW`], or [`WO`]).
/// - `T`: The type of the value being pointed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<A, T>
where
    A: VolatilePtrAccess<T>,
{
    /// The actual hardware memory address.
    address: A::TPtr,
    /// Zero-sized marker to tie the struct to a specific access level (RO/RW/WO).
    _access_marker: PhantomData<A>,
    /// Zero-sized marker to keep track of the underlying data type `T`.
    _type_marker: PhantomData<T>,
}

impl<T> VolatilePtr<RW, T> {
    // RW
    /// Creates a pointer from a memory address.
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    /// Performs a volatile read.
    #[inline]
    pub unsafe fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    /// Performs a volatile write.
    #[inline]
    pub unsafe fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.address, value) }
    }

    /// Modifies the value using a closure (Read-Modify-Write).
    #[inline]
    pub unsafe fn set(&self, set_fn: impl FnOnce(T) -> T) {
        unsafe {
            let mut v = self.read();
            v = set_fn(v);
            self.write(v);
        }
    }

    /// Casts the internal address to a raw mutable pointer of another type.
    #[inline]
    pub const fn cast<N>(&self) -> *mut N {
        self.address as _
    }

    /// Calculates an offset from the pointer using typed arithmetic.
    /// 
    /// The offset is multiplied by `size_of::<T>()`.
    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    /// Calculates a byte-based offset from the pointer.
    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    /// Calculates a byte-based offset using wrapping arithmetic.
    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }

    /// Spins until the register matches the expected value.
    #[inline]
    pub unsafe fn wait_until(&self, value: T, mut spin_hint: impl FnMut())
    where
        T: PartialEq + Copy,
    {
        while unsafe { self.read() } != value {
            spin_hint();
        }
    }
}

impl<T> VolatilePtr<RW, T>
where
    T: core::ops::BitOr<Output = T>
        + core::ops::BitAnd<Output = T>
        + core::ops::Not<Output = T>
        + Copy,
{
    // RW

    /// Sets bits using a bitwise mask (`v | mask`).
    #[inline]
    pub unsafe fn set_bits(&self, mask: T) {
        unsafe { self.set(|v| v | mask) };
    }

    /// Clears bits using a bitwise mask (`v & !mask`).
    #[inline]
    pub unsafe fn clear_bits(&self, mask: T) {
        unsafe { self.set(|v| v & !mask) };
    }
}

impl<T> VolatilePtr<RO, T> {
    // RO
    /// Creates a pointer from a memory address.
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    /// Performs a volatile read.
    #[inline]
    pub unsafe fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    /// Casts the internal address to a raw mutable pointer of another type.
    #[inline]
    pub const fn cast<N>(&self) -> *const N {
        self.address as _
    }

    /// Calculates an offset from the pointer using typed arithmetic.
    /// 
    /// The offset is multiplied by `size_of::<T>()`.
    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    /// Calculates a byte-based offset from the pointer.
    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    /// Calculates a byte-based offset using wrapping arithmetic.
    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }

    /// Spins until the register matches the expected value.
    #[inline]
    pub unsafe fn wait_until(&self, value: T, mut spin_hint: impl FnMut())
    where
        T: PartialEq + Copy,
    {
        while unsafe { self.read() } != value {
            spin_hint();
        }
    }
}

impl<T> VolatilePtr<WO, T> {
    // WO
    /// Creates a pointer from a memory address.
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    /// Performs a volatile write.
    #[inline]
    pub unsafe fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.address, value) }
    }

    /// Casts the internal address to a raw mutable pointer of another type.
    #[inline]
    pub const fn cast<N>(&self) -> *mut N {
        self.address as _
    }

    /// Calculates an offset from the pointer using typed arithmetic.
    /// 
    /// The offset is multiplied by `size_of::<T>()`.
    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    /// Calculates a byte-based offset from the pointer.
    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    /// Calculates a byte-based offset using wrapping arithmetic.
    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }
}

impl<A, T> VolatilePtr<A, T>
where
    A: VolatilePtrAccess<T>,
{
    /// Creates a new `VolatilePtr` from a raw pointer of the appropriate type.
    #[inline]
    pub const fn from_ptr(address: A::TPtr) -> Self {
        Self {
            address: address,
            _access_marker: PhantomData,
            _type_marker: PhantomData,
        }
    }

    /// Returns the underlying raw pointer.
    #[inline]
    pub const fn get_address(&self) -> A::TPtr {
        self.address
    }

    /// Returns the raw memory address as a `usize`.
    #[inline]
    pub const fn get_usize_address(&self) -> usize {
        unsafe { crate::access::into_usize_unchecked::<A, T>(self.address) }
    }

    /// Returns the underlying raw pointer.
    #[inline]
    pub const fn as_raw_ptr(&self) -> A::TPtr {
        self.get_address()
    }
}
