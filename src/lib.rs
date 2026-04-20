#![no_std]

pub mod access;
pub mod ptr;

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
