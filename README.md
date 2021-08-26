# mycobot-rs
[![crates.io](https://img.shields.io/crates/v/mycobot.svg)](https://crates.io/crates/mycobot)

MyCobot API in Rust.
## Getting started

```rust
use mycobot::*;
use std::io;

pub fn main() -> Result<(), io::Error> {
    let mut mycobot = MyCobotSerialOperator::new("/dev/ttyUSB0", 115200);
    mycobot.send_angles(&vec![0.0, 0.0, 0.0, 0.0, 30.0, 0.0], 50)?;
    Ok(())
}
```