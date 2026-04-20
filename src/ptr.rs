use crate::access::{RO, RW, VolatilePtrAccess, WO};
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<A, T>
where
    A: VolatilePtrAccess<T>,
{
    address: A::TPtr,
    _access_marker: PhantomData<A>,
    _type_marker: PhantomData<T>,
}

impl<T> VolatilePtr<RW, T> {
    // RW
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    #[inline]
    pub unsafe fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    #[inline]
    pub unsafe fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.address, value) }
    }

    #[inline]
    pub unsafe fn set(&self, set_fn: impl FnOnce(T) -> T) {
        unsafe {
            let mut v = self.read();
            v = set_fn(v);
            self.write(v);
        }
    }

    #[inline]
    pub const fn cast<N>(&self) -> *mut N {
        self.address as _
    }

    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }

    #[inline]
    pub const fn get_address_and_cast<NT>(&self) -> *const NT {
        self.address as _
    }
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
    #[inline]
    pub unsafe fn set_bits(&self, mask: T) {
        unsafe { self.set(|v| v | mask) };
    }

    #[inline]
    pub unsafe fn clear_bits(&self, mask: T) {
        unsafe { self.set(|v| v & !mask) };
    }
}

impl<T> VolatilePtr<RO, T> {
    // RO
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    #[inline]
    pub unsafe fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    #[inline]
    pub const fn cast<N>(&self) -> *const N {
        self.address as _
    }

    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }

    #[inline]
    pub const fn get_address_and_cast<NT>(&self) -> *const NT {
        self.address as _
    }

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
    #[inline]
    pub const fn from_usize(address: usize) -> Self {
        Self::from_ptr(address as _)
    }

    #[inline]
    pub unsafe fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.address, value) }
    }

    #[inline]
    pub const fn cast<N>(&self) -> *mut N {
        self.address as _
    }

    #[inline]
    pub const unsafe fn add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().add(count) })
    }

    #[inline]
    pub const unsafe fn byte_add(&self, count: usize) -> Self {
        Self::from_ptr(unsafe { self.get_address().byte_add(count) })
    }

    #[inline]
    pub const unsafe fn raw_byte_add(&self, count: usize) -> Self {
        let ptr = self.get_address() as *mut u8;
        let offset_ptr = ptr.wrapping_add(count);
        Self::from_ptr(offset_ptr as *mut T)
    }

    #[inline]
    pub const fn get_address_and_cast<NT>(&self) -> *const NT {
        self.address as _
    }
}

impl<A, T> VolatilePtr<A, T>
where
    A: VolatilePtrAccess<T>,
{
    #[inline]
    pub const fn from_ptr(address: A::TPtr) -> Self {
        Self {
            address: address,
            _access_marker: PhantomData,
            _type_marker: PhantomData,
        }
    }

    #[inline]
    pub const fn get_address(&self) -> A::TPtr {
        self.address
    }

    #[inline]
    pub const fn get_usize_address(&self) -> usize {
        unsafe { crate::access::into_usize_unchecked::<A, T>(self.address) }
    }

    #[inline]
    pub const fn as_raw_ptr(&self) -> A::TPtr {
        self.address
    }
}
