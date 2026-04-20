use volatile_table::volatile_table;

volatile_table! {
    map[rw <u32> AO_BASE = 0xC81004C0]: {
        wo <u32> AO_WFIFO   += 0 << 2; // Write FIFO
        ro <u32> AO_RFIFO   += 1 << 2; // Read FIFO
        rw <u32> AO_CONTROL += 2 << 2; // Control Register
        ro <u32> AO_STATUS  += 3 << 2; // Status Register
        rw <u32> AO_MISC    += 4 << 2; // Misc Settings
    }
}
const TX_FIFO_FULL: u32 = 0x200000;

pub fn write_byte(a: u8) {
    unsafe {
        while (AO_STATUS.read() & TX_FIFO_FULL) != 0 {} // (busy-wait loop)
        AO_WFIFO.write(a as _);
    }
}

pub fn print_message(msg: &str) {
    for byte in msg.as_bytes() {
        write_byte(*byte);
    }
}

pub fn main() {
    print_message("Hello, world!\r\n");
}
