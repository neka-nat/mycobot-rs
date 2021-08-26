use mycobot::*;
use std::env;
use std::io;

pub fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut mycobot = MyCobotSerialOperator::new(&args[0], 115200);
    let target = vec![-80.0, -10.0, -130.0, 70.0, 0.0, 0.0];
    mycobot.set_color(255, 0, 0)?;
    mycobot.sync_send_angles(&target, 50, 3.0)?;
    let mut coords = mycobot.get_coords()?;
    println!("Coords: {:?}", coords);
    coords[2] -= 10.0;
    mycobot.sync_send_coords(&coords, 50, Mode::Normal, 3.0)?;
    let mut coords = mycobot.get_coords()?;
    println!("Coords: {:?}", coords);
    coords[2] -= 10.0;
    mycobot.sync_send_coords(&coords, 50, Mode::Normal, 3.0)?;
    let coords = mycobot.get_coords()?;
    println!("Coords: {:?}", coords);
    mycobot.set_color(0, 255, 0)?;
    Ok(())
}
