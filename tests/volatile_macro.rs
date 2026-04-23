#[test]
fn test_volatile_macro() {
    use volatile_table::volatile;

    let mut v = 10usize;
    let addr = &mut v as *mut _;

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
