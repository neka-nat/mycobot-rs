# mycobot-rs

Mycobot API in Rust.
## Getting started

```rust
use mycobot::*;
use std::io;
use std::env;


pub fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let serial = Serial::new(&args[0], 115200);
    let mut mycobot = MyCobotOperator::new(serial);
    mycobot.send_angles(&vec![0.0, 0.0, 0.0, 0.0, 30.0, 0.0], 50)?;
    Ok(())
}
```