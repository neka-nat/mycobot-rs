# mycobot-rs
[![crates.io](https://img.shields.io/crates/v/mycobot.svg)](https://crates.io/crates/mycobot)

MyCobot API in Rust.
## Getting started

```rust
use mycobot::*;

pub fn main() -> Result<()> {
    let mut mycobot = MyCobotSerialOperator::new("/dev/ttyUSB0", 115200);
    mycobot.send_angles(&[0.0, 0.0, 0.0, 0.0, 30.0, 0.0], 50)?;
    Ok(())
}
```

## Demo

Run example.

```sh
sudo chmod 666 /dev/ttyUSB0
cargo run --release --example send_coords /dev/ttyUSB0
```

![send_coords](https://raw.githubusercontent.com/neka-nat/mycobot-rs/master/assets/send_coords.gif)
