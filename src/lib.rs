//Copyright 2026 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #UlinProject 2026

#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

pub mod access;
pub mod ptr;

/// A helper macro to resolve volatile pointer types and access rights.
///
/// This macro maps shorthand syntax (like `rw`, `ro`, `wo`) and data types 
/// to the underlying [`VolatilePtr`] implementation. It is the core engine 
/// behind the type generation in `volatile_table!`.
///
/// # Syntax Patterns:
///
/// - `[ <$t:ty> ]`       => Read-Write pointer to type `$t`.
/// - `[ rw <$t:ty> ]`    => Read-Write pointer to type `$t`.
/// - `[ ro <$t:ty> ]`    => Read-Only pointer to type `$t`.
/// - `[ wo <$t:ty> ]`    => Write-Only pointer to type `$t`.
/// - `[ rw ]`, `[ ro ]`, `[ wo ]` => Access-specific pointer to `usize` (default).
/// - `[]`                => Defaults to `rw <usize>`.
///
/// # Examples
///
/// ```rust
/// use volatile_table::volatile_type;
///
/// // Resolves to VolatilePtr<RW, u32>
/// type ControlReg = volatile_type![rw <u32>];
///
/// // Resolves to VolatilePtr<RO, usize>
/// type StatusReg = volatile_type![ro];
/// ```
#[macro_export]
macro_rules! volatile_type {
    [ <$t: ty> ] => {
        $crate::ptr::VolatilePtr<$crate::access::RW, $t>
    };
    [ rw <$t: ty> ] => {
        $crate::ptr::VolatilePtr<$crate::access::RW, $t>
    };
    [ ro <$t: ty> ] => {
        $crate::ptr::VolatilePtr<$crate::access::RO, $t>
    };

    [ wo <$t: ty> ] => {
        $crate::ptr::VolatilePtr<$crate::access::WO, $t>
    };

    [ rw ] => {
        $crate::ptr::VolatilePtr<$crate::access::RW, usize>
    };
    [ ro ] => {
        $crate::ptr::VolatilePtr<$crate::access::RO, usize>
    };

    [ wo ] => {
        $crate::ptr::VolatilePtr<$crate::access::WO, usize>
    };

    [] => {
        $crate::volatile_type!(rw)
    };
}

/// A universal constructor macro for creating volatile pointers.
///
/// This macro provides a flexible DSL for instantiating [`VolatilePtr`] from raw pointers 
/// or memory addresses (as `usize`). It handles access rights and type casting automatically.
///
/// # Syntax Variations:
///
/// 1. **Address-based (from_usize)**: Starts with `@from_usize`.
/// 2. **Pointer-based (default)**: Uses `from_ptr` internally.
/// 3. **Typed**: Use `<T>` to specify the data type.
/// 4. **Access-controlled**: Use `rw`, `ro`, or `wo` to set permissions.
///
/// # Patterns & Examples:
///
/// ```rust
/// use volatile_table::volatile;
///
/// // 1. Default (Read-Write, usize, from raw pointer)
/// let p = volatile!(&raw mut SOME_VAR);
///
/// // 2. Custom Type and Access (Read-Only, u32)
/// let p = volatile!(ro <u32> : &raw mut SOME_VAR);
///
/// // 3. From a literal address (common in embedded)
/// // This uses the @from_usize flag
/// let p = volatile!(@from_usize rw <u32> : 0x4000_1000);
///
/// // 4. Shorthand for typed RW
/// let p = volatile!(<u8> : &raw mut SOME_VAR);
/// ```
///
/// # Internal flags:
/// - `@from_usize`: Forces the use of usize as a pointer
/// - `@ignore_from_usize`: Flag that excludes the previous `@from_usize` flag; (used internally only.)
#[macro_export]
macro_rules! volatile {
    [ $(@from_usize)? @ignore_from_usize $access:ident <$ty:ty> : $($a:tt)+ ] => {
        <$crate::volatile_type!($access <$ty>)>::from_ptr($($a)*)
    };
    [ $(@from_usize)? @ignore_from_usize $tt:ident : $($a:tt)+ ] => {
        <$crate::volatile_type!($tt)>::from_ptr($($a)*)
    };
    [ $(@from_usize)? @ignore_from_usize $ty:ty : $($a:tt)+ ] => {
        <$crate::volatile_type!(<$ty>)>::from_ptr($($a)*)
    };
    [ $(@from_usize)? @ignore_from_usize $($a:tt)+ ] => {
        <$crate::volatile_type!(rw)>::from_ptr($($a)*)
    };


    [ @from_usize $access:ident <$ty:ty> : $($a:tt)+ ] => {
        <$crate::volatile_type!($access <$ty>)>::from_usize($($a)*)
    };
    [ @from_usize $tt:ident : $($a:tt)+ ] => {
        <$crate::volatile_type!($tt)>::from_usize($($a)*)
    };
    [ @from_usize <$ty:ty> : $($a:tt)+ ] => {
        <$crate::volatile_type!(<$ty>)>::from_usize($($a)*)
    };
    [ @from_usize $($a:tt)+ ] => {
        <$crate::volatile_type!(rw)>::from_usize($($a)*)
    };


    [ $access:ident <$ty:ty> : $($a:tt)+ ] => {
        <$crate::volatile_type!($access <$ty>)>::from_ptr($($a)*)
    };
    [ $tt:ident : $($a:tt)+ ] => {
        <$crate::volatile_type!($tt)>::from_ptr($($a)*)
    };
    [ $ty:ty : $($a:tt)+ ] => {
        <$crate::volatile_type!(<$ty>)>::from_ptr($($a)*)
    };
    [ $($a:tt)+ ] => {
        <$crate::volatile_type!(rw)>::from_ptr($($a)*)
    };
}

/// The main macro for defining volatile register tables and memory maps.
///
/// `volatile_table!` allows you to declare memory-mapped registers as typed constants
/// and create logical groups (maps) with relative offsets.
///
/// # Syntax Patterns
///
/// 1. **Direct Declaration**: `access <type> NAME = address;`
///    - Declares a `const` volatile pointer.
///    - If `<type>` is omitted, it defaults to `usize`.
///    - Addresses are automatically handled as `usize` via `@from_usize` logic.
///
/// 2. **Memory Mapping**: `map [BASE_REG]: { OFFSET_REGS }`
///    - Defines a set of registers relative to a base address.
///    - Automatically calculates addresses using the `+=` syntax inside the block.
///
/// 3. **Metadata & Visibility**: Supports doc comments `///` and visibility modifiers (e.g., `pub`).
///
/// # Examples
///
/// ### Simple Register Definition
/// ```rust
/// volatile_table! {
///     /// System clock control
///     pub rw <u32> CLK_CTRL = 0x4000_0000;
///     /// Status register (read-only)
///     ro STATUS = 0x4000_0004; 
/// }
/// ```
///
/// ### Advanced Memory Map (e.g., UART Driver)
/// ```rust
/// volatile_table! {
///     map [pub rw <u32> UART_BASE = 0xC810_04C0]: {
///         wo <u32> TX_FIFO += 0x00;
///         ro <u32> RX_FIFO += 0x04;
///         rw <u32> CONTROL += 0x08;
///     }
/// }
/// ```
///
/// # How it works
/// - For entries with `<type>`, the macro assumes the initialization value is a memory address (`usize`).
/// - For entries without `<type>`, it treats the value as a pointer expression unless flagged.
/// - The `map` block expands into a series of constant definitions where each child register 
///   is calculated as `BASE_ADDR + OFFSET`.
#[macro_export]
macro_rules! volatile_table {
    [
        $(#[$meta:meta])*
        $vis:vis $access_type:ident $n: ident $(= $(@$addition_exprflag:tt)? $e: expr)? $(
            ;
            $($all:tt)*
        )?
    ] => { // FROM_TYPE
        $(#[$meta])*
        $vis const $n:   $crate::volatile_type!($access_type) $(= $crate::volatile!($(@$addition_exprflag)? $access_type: $e))?;

        $(
            $crate::volatile_table!{
                $($all)*
            }
        )?
    };
    [
        $(#[$meta:meta])*
        $vis:vis $access_type:ident <$ty: ty> $n: ident $(= $(@$addition_exprflag:tt)? $e: expr)? $(
            ;
            $($all:tt)*
        )?
    ] => { // FROM_USIZE
        $(#[$meta])*
        $vis const $n:   $crate::volatile_type!($access_type <$ty>) $(= $crate::volatile!(@from_usize $(@$addition_exprflag)? $access_type <$ty>: $e))?;

        $(
            $crate::volatile_table!{
                $($all)*
            }
        )?
    };
    [
        map [
            $(#[$meta:meta])*
            $vis:vis $access_type:ident $n: ident $(= $(@$addition_exprflag:tt)? $e: expr)?
        ]: {
            $($map_code:tt)*
        } $(
            ;
            $($all:tt)*
        )?
    ] => { // FROM_TYPE
        $crate::_volatile_map! {
            [$n]: {
                $($map_code)*
            }
        }

        $crate::volatile_table! {
            $(#[$meta])*
            $vis $access_type $n $(= $(@$addition_exprflag)? $e)?

            $(; $($all)*)?
        }
    };
    [
        map [
            $(#[$meta:meta])*
            $vis:vis $access_type:ident <$ty: ty> $n: ident $(= $(@$addition_exprflag:tt)? $e: expr)?
        ]: {
            $($map_code:tt)*
        } $(
            ;
            $($all:tt)*
        )?
    ] => { // FROM_USIZE
        $crate::_volatile_map! {
            [$n]: {
                $($map_code)*
            }
        }
        $crate::volatile_table! {
            $(#[$meta])*
            $vis $access_type <$ty> $n $(= $(@$addition_exprflag)? $e)?

            $(; $($all)*)?
        }
    };
    [
        map [
            $n: path
        ]: {
            $($map_code:tt)*
        } $(
            ;
            $($all:tt)*
        )?
    ] => { // FROM_PATH
        $crate::_volatile_map! {
            [$n]: {
                $($map_code)*
            }
        }
        $(
            $crate::volatile_table! {
                $($all)*
            }
        )?
    };
    [ ] => [];
}

/// **Internal use only.** 
///
/// This macro handles the expansion of the `map` block within [`volatile_table!`].
/// It recursively processes register definitions and calculates their absolute 
/// addresses based on a base pointer using byte offsets.
#[doc(hidden)]
#[macro_export]
macro_rules! _volatile_map {
    [
        [$map_name: expr]: {
            $(#[$meta:meta])*
            $vis:vis $access_type:ident $n: ident $(+= $(@$addition_exprflag:tt)? $e: expr)? $(
                ;
                $($all:tt)*
            )?
        } $(;)?
    ] => { // FROM_TYPE
        $crate::volatile_table! {
            $(#[$meta])*
            $vis $access_type $n $(= @ignore_from_usize $(@$addition_exprflag)? unsafe {
                $map_name.raw_byte_add($e).as_raw_ptr()
            })?;
        }

        $(
            $crate::_volatile_map! {
                [$map_name]: {
                    $($all)*
                }
            }
        )?
    };
    [
        [$map_name: expr]: {
            $(#[$meta:meta])*
            $vis:vis $access_type:ident <$ty: ty> $n: ident $(+= $(@$addition_exprflag:tt)? $e: expr)? $(
                ;
                $($all:tt)*
            )?
        } $(;)?
    ] => { // FROM_USIZE
        $crate::volatile_table! {
            $(#[$meta])*
            $vis $access_type <$ty> $n $(= @ignore_from_usize $(@$addition_exprflag)? unsafe {
                $map_name.raw_byte_add($e).as_raw_ptr()
            })?;
        }

        $(
            $crate::_volatile_map! {
                [$map_name]: {
                    $($all)*
                }
            }
        )?
    };

    [
        [$map_name: expr]: {} $(;)?
    ] => {};
    [ ] => [];
}

#[cfg(test)]
#[test]
fn _test_volatile_macro() {
    let mut v = 10usize;
    let addr = &mut v as *mut _ as *mut usize;

    unsafe {
        // rw
        let volatile_ptr = volatile!(addr);
        assert_eq!(volatile_ptr.read(), 10);
        volatile_ptr.write(1);
        assert_eq!(volatile_ptr.read(), 1);
    }

    unsafe {
        // rw2
        let volatile_ptr = volatile!(rw: addr);
        volatile_ptr.write(2);
        assert_eq!(volatile_ptr.read(), 2);
    }

    unsafe {
        // wo
        let volatile_ptr = volatile!(wo: addr);
        volatile_ptr.write(3);
    }

    unsafe {
        // ro
        let volatile_ptr = volatile!(ro: addr);
        assert_eq!(volatile_ptr.read(), 3);
    }
}

#[cfg(test)]
#[test]
fn _test_volatile_table_macro() {
    static mut V: usize = 0usize;
    unsafe {
        V = 10usize;
    }

    volatile_table! {
        rw RW_V_PTR = &raw mut V;
        ro RO_V_PTR = &raw mut V;
        wo WO_V_PTR = &raw mut V;

        map[RW_V_PTR]: {
            #[allow(unused)]
            rw TEST += 0;
            #[allow(unused)]
            rw TEST3 += 0;
            #[allow(unused)]
            rw TEST2 += 1;
        };

        map[
            #[allow(unused)]
            rw RW2_V_PTR = &raw mut V
        ]: {
            #[allow(unused)]
            rw TEST44 += 0;
        };
    }

    {
        let volatile_ptr = RW_V_PTR;
        unsafe {
            // rw
            assert_eq!(volatile_ptr.read(), 10);
            volatile_ptr.write(1);
            assert_eq!(volatile_ptr.read(), 1);
        }

        unsafe {
            // rw2
            volatile_ptr.write(2);
            assert_eq!(volatile_ptr.read(), 2);
        }

        unsafe {
            // wo
            volatile_ptr.write(3);
        }

        unsafe {
            // ro
            assert_eq!(volatile_ptr.read(), 3);
        }
    }

    unsafe {
        WO_V_PTR.write(4);
    }

    unsafe {
        assert_eq!(RO_V_PTR.read(), 4);
    }
}
