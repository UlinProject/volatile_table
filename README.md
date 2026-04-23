<div id="header" align="center">

  <b>[volatile_table]</b>
  
  (A zero-cost, type-safe DSL for MMIO and volatile register mapping with compile-time access control (RO/RW/WO).)
  </br></br>

<div id="badges">
  <a href="./LICENSE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/volatile_table">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/volatile_table">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
	
  [![CI](https://github.com/UlinProject/volatile_table/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/UlinProject/volatile_table/actions/workflows/CI.yml) 


</div>
</div>

## Why volatile_table?
- Type-Safe Access: Compiler-enforced RO/RW/WO rights.
- Hardware-Friendly DSL: Use byte offsets (+=) directly from datasheets.
- Zero-Overhead: No runtime cost, just direct volatile instructions.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
volatile_table = "0.0.1"
```

and this to your source code:

```rust
use volatile_table::volatile_table;
```

## Example

```rust
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
    // Note: .read() and .write() operations are unsafe because they perform direct memory access,
    // which is necessary for hardware interaction.
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
```

<a href="./examples">
  See all
</a>

## License

This project has a single license (LICENSE-APACHE-2.0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2026 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License
<div align="left">
  <a href="./LICENSE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
