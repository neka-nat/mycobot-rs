use mycobot::*;
use std::env;

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut mycobot = MyCobotSerialOperator::new(&args[0], 115200);
    let target = [0.0, 0.0, 0.0, 0.0, 50.0, 0.0];
    mycobot.set_color(255, 0, 0)?;
    mycobot.sync_send_angles(&target, 50, 10.0)?;
    let angles = mycobot.get_angles()?;
    println!("Angles: {:?}", angles);
    mycobot.set_color(0, 255, 0)?;
    Ok(())
}
