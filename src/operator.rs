use super::common::*;
use super::io::{Connection, Serial};
use super::utils::*;
use std::io;
use std::marker::PhantomData;

pub struct MyCobotOperator<T: Connection> {
    connection: T,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Connection> MyCobotOperator<T> {
    pub fn from_connection(connection: T) -> MyCobotOperator<T> {
        MyCobotOperator {
            connection,
            _marker: PhantomData,
        }
    }
    fn concat_message(genre: u8, command_data: &[u8]) -> Vec<u8> {
        let len = 2 + command_data.len();
        let header = [Command::HEADER, Command::HEADER, len as u8, genre];
        [&header[..], command_data, &[Command::FOOTER]].concat()
    }
    fn is_frame_header(data: &[u8], pos: usize) -> bool {
        data[pos] == Command::HEADER && data[pos + 1] == Command::HEADER
    }
    fn process_received(data: &[u8], genre: u8) -> Vec<i16> {
        if data.is_empty() {
            return Vec::new();
        }
        let some_idx =
            (0..(data.len() - 1)).position(|i| MyCobotOperator::<T>::is_frame_header(data, i));
        if let Some(idx) = some_idx {
            let data_len = (data[idx + 2] - 2) as usize;
            let cmd_id = data[idx + 3];
            if cmd_id != genre {
                return Vec::new();
            }
            let data_pos = idx + 4;
            let valid_data = &data[data_pos..(data_pos + data_len)];
            match data_len {
                12 => decode_int16_vec(valid_data),
                2 => {
                    if genre == Command::IS_SERVO_ENABLE {
                        [decode_int8(&valid_data[1..2]) as i16].to_vec()
                    } else {
                        [decode_int16(valid_data)].to_vec()
                    }
                }
                _ => [decode_int8(valid_data) as i16].to_vec(),
            }
        } else {
            Vec::new()
        }
    }
    fn write_command(&mut self, genre: u8, command_data: &[u8]) -> Result<(), io::Error> {
        let command = MyCobotOperator::<T>::concat_message(genre, command_data);
        self.connection.write(&command)
    }
    fn write_command_and_receive(
        &mut self,
        genre: u8,
        command_data: &[u8],
    ) -> Result<Vec<i16>, io::Error> {
        let command = MyCobotOperator::<T>::concat_message(genre, command_data);
        let res = self.connection.write_and_read(&command)?;
        Ok(MyCobotOperator::<T>::process_received(&res, genre))
    }
    pub fn version(&mut self) -> Result<String, io::Error> {
        let command = MyCobotOperator::<T>::concat_message(Command::VERSION, &Vec::<u8>::new());
        let res = self.connection.write_and_read(&command)?;
        let version = res.iter().map(|&s| s as char).collect::<String>();
        Ok(version)
    }
    pub fn power_on(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::POWER_ON, &[])
    }
    pub fn power_off(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::POWER_OFF, &[])
    }
    pub fn is_power_on(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_POWER_ON, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn release_all_servos(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::RELEASE_ALL_SERVOS, &[])
    }
    pub fn is_controller_connected(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_CONTROLLER_CONNECTED, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn get_angles(&mut self) -> Result<Vec<f64>, io::Error> {
        let res = self.write_command_and_receive(Command::GET_ANGLES, &[])?;
        Ok(res.into_iter().map(int_to_angle).collect::<Vec<_>>())
    }
    pub fn send_angle(&mut self, id: Angle, degree: f64, speed: u8) -> Result<(), io::Error> {
        let command_data = [
            &[id as u8],
            &encode_int16(angle_to_int(degree))[..],
            &[speed],
        ]
        .concat();
        self.write_command(Command::SEND_ANGLE, &command_data)
    }
    pub fn send_angles(&mut self, degrees: &[f64], speed: u8) -> Result<(), io::Error> {
        let command_data = [
            &encode_int16_vec(
                &degrees
                    .iter()
                    .map(|deg| angle_to_int(*deg))
                    .collect::<Vec<_>>()[..],
            )[..],
            &[speed],
        ]
        .concat();
        self.write_command(Command::SEND_ANGLES, &command_data)
    }
    pub fn get_coords(&mut self) -> Result<Vec<f64>, io::Error> {
        let res = self.write_command_and_receive(Command::GET_COORDS, &[])?;
        Ok(int_vec_to_coords(&res))
    }
    pub fn send_coord(&mut self, id: Coord, coord: f64, speed: u8) -> Result<(), io::Error> {
        let command_data = [
            &[id as u8 - 1],
            &encode_int16(coord_to_int(coord))[..],
            &[speed],
        ]
        .concat();
        self.write_command(Command::SEND_COORD, &command_data)
    }
    pub fn send_coords(&mut self, coords: &[f64], speed: u8, mode: u8) -> Result<(), io::Error> {
        let command_data = [
            &encode_int16_vec(&coords_to_int_vec(coords))[..],
            &[speed],
            &[mode],
        ]
        .concat();
        self.write_command(Command::SEND_COORDS, &command_data)
    }
    pub fn is_in_angle_position(&mut self, degrees: &[f64]) -> Result<i32, io::Error> {
        let command_data = [
            &encode_int16_vec(
                &degrees
                    .iter()
                    .map(|deg| angle_to_int(*deg))
                    .collect::<Vec<_>>()[..],
            )[..],
            &[0u8],
        ]
        .concat();
        let res = self.write_command_and_receive(Command::IS_IN_POSITION, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn is_in_coord_position(&mut self, coords: &[f64]) -> Result<i32, io::Error> {
        let command_data = [&encode_int16_vec(&coords_to_int_vec(coords))[..], &[1u8]].concat();
        let res = self.write_command_and_receive(Command::IS_IN_POSITION, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn is_moving(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_MOVING, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn jog_angle(
        &mut self,
        id: Angle,
        direction: Direction,
        speed: u8,
    ) -> Result<(), io::Error> {
        let command_data = [id as u8, direction as u8, speed];
        self.write_command(Command::JOG_ANGLE, &command_data)
    }
    pub fn jog_coord(
        &mut self,
        id: Coord,
        direction: Direction,
        speed: u8,
    ) -> Result<(), io::Error> {
        let command_data = [id as u8, direction as u8, speed];
        self.write_command(Command::JOG_COORD, &command_data)
    }
    pub fn jog_stop(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::JOG_STOP, &[])
    }
    pub fn pause(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::PAUSE, &[])
    }
    pub fn is_paused(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_PAUSED, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn resume(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::RESUME, &[])
    }
    pub fn stop(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::STOP, &[])
    }
    pub fn set_encoder(&mut self, id: Angle, encoder: i16) -> Result<(), io::Error> {
        let command_data = [&[id as u8], &encode_int16(encoder)[..]].concat();
        self.write_command(Command::SET_ENCODER, &command_data)
    }
    pub fn get_encoder(&mut self, id: Angle) -> Result<i32, io::Error> {
        let command_data = [id as u8];
        let res = self.write_command_and_receive(Command::GET_ENCODER, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn set_encoders(&mut self, encoders: &[i16], sp: u8) -> Result<(), io::Error> {
        let command_data = [&encode_int16_vec(encoders)[..], &[sp]].concat();
        self.write_command(Command::SET_ENCODERS, &command_data)
    }
    pub fn get_encoders(&mut self) -> Result<Vec<i16>, io::Error> {
        self.write_command_and_receive(Command::GET_ENCODERS, &[])
    }
    pub fn get_speed(&mut self) -> Result<Vec<i16>, io::Error> {
        self.write_command_and_receive(Command::GET_SPEED, &[])
    }
    pub fn set_speed(&mut self, speed: u8) -> Result<(), io::Error> {
        let command_data = [speed];
        self.write_command(Command::SET_SPEED, &command_data)
    }
    pub fn get_joint_min_angle(&mut self, id: Angle) -> Result<Vec<i16>, io::Error> {
        let command_data = [id as u8];
        self.write_command_and_receive(Command::GET_JOINT_MIN_ANGLE, &command_data)
    }
    pub fn get_joint_max_angle(&mut self, id: Angle) -> Result<Vec<i16>, io::Error> {
        let command_data = [id as u8];
        self.write_command_and_receive(Command::GET_JOINT_MAX_ANGLE, &command_data)
    }
    pub fn is_servo_enable(&mut self, id: Angle) -> Result<i32, io::Error> {
        let command_data = [id as u8];
        let res = self.write_command_and_receive(Command::IS_SERVO_ENABLE, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn is_all_servo_enable(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_ALL_SERVO_ENABLE, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn set_servo_data(
        &mut self,
        servo_no: u8,
        data_id: u8,
        value: u8,
    ) -> Result<(), io::Error> {
        let command_data = [servo_no, data_id, value];
        self.write_command(Command::SET_SERVO_DATA, &command_data)
    }
    pub fn get_servo_data(&mut self, servo_no: u8, data_id: u8) -> Result<Vec<i16>, io::Error> {
        let command_data = [servo_no, data_id];
        self.write_command_and_receive(Command::GET_SERVO_DATA, &command_data)
    }
    pub fn set_servo_calibration(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::SET_SERVO_CALIBRATION, &[])
    }
    pub fn release_servo(&mut self, servo_id: Angle) -> Result<(), io::Error> {
        let command_data = [servo_id as u8];
        self.write_command(Command::RELEASE_SERVO, &command_data)
    }
    pub fn focus_servo(&mut self, servo_id: Angle) -> Result<(), io::Error> {
        let command_data = [servo_id as u8];
        self.write_command(Command::FOCUS_SERVO, &command_data)
    }
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), io::Error> {
        let command_data = [r, g, b];
        self.write_command(Command::SET_COLOR, &command_data)
    }
    pub fn set_pin_mode(&mut self, pin_no: u8, pin_mode: PinMode) -> Result<(), io::Error> {
        let command_data = [pin_no, pin_mode as u8];
        self.write_command(Command::SET_PIN_MODE, &command_data)
    }
    pub fn set_digital_output(&mut self, pin_no: u8, pin_signal: bool) -> Result<(), io::Error> {
        let command_data = [pin_no, pin_signal as u8];
        self.write_command(Command::SET_DIGITAL_OUTPUT, &command_data)
    }
    pub fn get_digital_intput(&mut self, pin_no: u8) -> Result<i32, io::Error> {
        let command_data = [pin_no];
        let res = self.write_command_and_receive(Command::GET_DIGITAL_INPUT, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn set_pwm_output(&mut self, channel: u8, frequency: i16, pin_val: u8) -> Result<(), io::Error> {
        let command_data = [&[channel], &encode_int16(frequency)[..], &[pin_val]].concat();
        self.write_command(Command::SET_PWM_OUTPUT, &command_data)
    }
    pub fn get_gripper_value(&mut self) -> Result<Vec<i16>, io::Error> {
        self.write_command_and_receive(Command::GET_DIGITAL_INPUT, &[])
    }
    pub fn set_gripper_state(&mut self, state: GripperState, speed: u8) -> Result<(), io::Error> {
        let command_data = [state as u8, speed];
        self.write_command(Command::SET_GRIPPER_STATE, &command_data)
    }
    pub fn set_gripper_value(&mut self, value: u8, speed: u8) -> Result<(), io::Error> {
        let command_data = [value, speed];
        self.write_command(Command::SET_GRIPPER_VALUE, &command_data)
    }
    pub fn set_gripper_ini(&mut self) -> Result<(), io::Error> {
        self.write_command(Command::SET_GRIPPER_INI, &[])
    }
    pub fn is_gripper_moving(&mut self) -> Result<i32, io::Error> {
        let res = self.write_command_and_receive(Command::IS_GRIPPER_MOVING, &[])?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
    pub fn set_basic_output(&mut self, pin_no: u8, pin_signal: bool) -> Result<(), io::Error> {
        let command_data = [pin_no, pin_signal as u8];
        self.write_command(Command::SET_BASIC_OUTPUT, &command_data)
    }
    pub fn get_basic_input(&mut self, pin_no: u8) -> Result<i32, io::Error> {
        let command_data = [pin_no];
        let res = self.write_command_and_receive(Command::GET_BASIC_INPUT, &command_data)?;
        Ok(if res.is_empty() { -1 } else { res[0] as i32 })
    }
}

pub type MyCobotSerialOperator = MyCobotOperator<Serial>;

impl MyCobotSerialOperator {
    pub fn new(port: &str, baudrate: u32) -> MyCobotSerialOperator {
        let connection = Serial::new(port, baudrate);
        MyCobotSerialOperator::from_connection(connection)
    }
}
