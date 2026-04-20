
#[cfg(test)]
#[test]
fn test_volatile_table_macro() {
    use volatile_table::volatile_table;
    
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
