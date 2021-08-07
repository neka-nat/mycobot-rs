pub enum Angle {
    J1 = 1,
    J2 = 2,
    J3 = 3,
    J4 = 4,
    J5 = 5,
    J6 = 6,
}

pub enum Coord {
    X = 1,
    Y = 2,
    Z = 3,
    Rx = 4,
    Ry = 5,
    Rz = 6,
}

pub enum Direction {
    Decrease = 0,
    Increase = 1,
}

pub enum Mode {
    Normal = 0,
    Angular = 1,
    Linear = 2,
}

#[non_exhaustive]
pub struct Command;

impl Command {
    pub const HEADER: u8 = 0xFE;
    pub const FOOTER: u8 = 0xFA;

    pub const VERSION: u8 = 0x00;

    pub const POWER_ON: u8 = 0x10;
    pub const POWER_OFF: u8 = 0x11;
    pub const IS_POWER_ON: u8 = 0x12;
    pub const RELEASE_ALL_SERVOS: u8 = 0x13;
    pub const IS_CONTROLLER_CONNECTED: u8 = 0x14;
    pub const READ_NEXT_ERROR: u8 = 0x15;
    pub const SET_FREE_MODE: u8 = 0x1A;
    pub const IS_FREE_MODE: u8 = 0x1B;

    pub const GET_ANGLES: u8 = 0x20;
    pub const SEND_ANGLE: u8 = 0x21;
    pub const SEND_ANGLES: u8 = 0x22;
    pub const GET_COORDS: u8 = 0x23;
    pub const SEND_COORD: u8 = 0x24;
    pub const SEND_COORDS: u8 = 0x25;
    pub const PAUSE: u8 = 0x26;
    pub const IS_PAUSED: u8 = 0x27;
    pub const RESUME: u8 = 0x28;
    pub const STOP: u8 = 0x29;
    pub const IS_IN_POSITION: u8 = 0x2A;
    pub const IS_MOVING: u8 = 0x2B;

    pub const JOG_ANGLE: u8 = 0x30;
    pub const JOG_COORD: u8 = 0x32;
    pub const JOG_STOP: u8 = 0x34;
    pub const SET_ENCODER: u8 = 0x3A;
    pub const GET_ENCODER: u8 = 0x3B;
    pub const SET_ENCODERS: u8 = 0x3C;
    pub const GET_ENCODERS: u8 = 0x3D;

    pub const GET_SPEED: u8 = 0x40;
    pub const SET_SPEED: u8 = 0x41;
    pub const GET_FEED_OVERRIDE: u8 = 0x42;
    pub const GET_ACCELERATION: u8 = 0x44;
    pub const GET_JOINT_MIN_ANGLE: u8 = 0x4A;
    pub const GET_JOINT_MAX_ANGLE: u8 = 0x4B;

    pub const IS_SERVO_ENABLE: u8 = 0x50;
    pub const IS_ALL_SERVO_ENABLE: u8 = 0x51;
    pub const SET_SERVO_DATA: u8 = 0x52;
    pub const GET_SERVO_DATA: u8 = 0x53;
    pub const SET_SERVO_CALIBRATION: u8 = 0x54;
    pub const RELEASE_SERVO: u8 = 0x56;
    pub const FOCUS_SERVO: u8 = 0x57;

    pub const SET_PIN_MODE: u8 = 0x60;
    pub const SET_DIGITAL_OUTPUT: u8 = 0x61;
    pub const GET_DIGITAL_OUTPUT: u8 = 0x62;
    pub const SET_PWM_MODE: u8 = 0x63;
    pub const GET_PWM_MODE: u8 = 0x64;
    pub const GET_GRIPPER_VALUE: u8 = 0x65;
    pub const SET_GRIPPER_STATE: u8 = 0x66;
    pub const SET_GRIPPER_VALUE: u8 = 0x67;
    pub const SET_GRIPPER_INI: u8 = 0x68;
    pub const IS_GRIPPER_MOVING: u8 = 0x69;
    pub const SET_COLOR: u8 = 0x6A;

    pub const SET_BASIC_OUTPUT: u8 = 0xA0;
    pub const GET_BASIC_INPUT: u8 = 0xA1;
}
