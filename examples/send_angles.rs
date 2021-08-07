use mycobot::*;
use std::env;
use std::io;

pub fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let serial = Serial::new(&args[0], 115200);
    let mut mycobot = MyCobotOperator::new(serial);
    mycobot.send_angles(&vec![0.0, 0.0, 0.0, 0.0, 50.0, 0.0], 50)?;
    //loop {
    //    if mycobot.is_moving()? == 0 {
    //        break;
    //    }
    //}
    let angles = mycobot.get_angles()?;
    println!("Angles: {:?}", angles);
    Ok(())
}
