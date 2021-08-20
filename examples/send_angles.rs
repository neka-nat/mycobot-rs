use mycobot::*;
use std::env;
use std::io;

pub fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut mycobot = MyCobotSerialOperator::new(&args[0], 115200);
    mycobot.send_angles(&vec![0.0, 0.0, 0.0, 0.0, 50.0, 0.0], 50)?;
    let angles = mycobot.get_angles()?;
    println!("Angles: {:?}", angles);
    Ok(())
}
