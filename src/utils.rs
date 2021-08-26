use byteorder::{BigEndian, ByteOrder};

pub fn angle_to_int(degree: f64) -> i16 {
    (degree * 100.0) as i16
}

pub fn coord_to_int(coord: f64) -> i16 {
    (coord * 10.0) as i16
}

pub fn int_to_angle(val: i16) -> f64 {
    (val as f64) / 100.0
}

pub fn int_to_coord(val: i16) -> f64 {
    (val as f64) / 10.0
}

pub fn coords_to_int_vec(coords: &[f64]) -> Vec<i16> {
    coords
        .iter()
        .enumerate()
        .map(|(i, coord)| {
            if i < 3 {
                coord_to_int(*coord)
            } else {
                angle_to_int(*coord)
            }
        })
        .collect()
}

pub fn int_vec_to_coords(vals: &[i16]) -> Vec<f64> {
    vals.iter()
        .enumerate()
        .map(|(i, v)| {
            if i < 3 {
                int_to_coord(*v)
            } else {
                int_to_angle(*v)
            }
        })
        .collect()
}

pub fn encode_int16(data: i16) -> [u8; 2] {
    let mut buf = [0u8; 2];
    BigEndian::write_i16(&mut buf, data);
    buf
}

pub fn encode_int16_vec(data: &[i16]) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.resize(data.len() * 2, 0);
    for i in 0..data.len() {
        BigEndian::write_i16(&mut buf[(2 * i)..(2 * i + 2)], data[i]);
    }
    buf
}

pub fn decode_int16(data: &[u8]) -> i16 {
    BigEndian::read_i16(&data[0..2])
}

pub fn decode_int8(data: &[u8]) -> i8 {
    i8::from_be_bytes([data[0]])
}

pub fn decode_int16_vec(data: &[u8]) -> Vec<i16> {
    let mut res = Vec::<i16>::new();
    for idx in (0..(data.len())).step_by(2) {
        res.push(BigEndian::read_i16(&data[idx..(idx + 2)]));
    }
    res
}

const MINANGLE: f64 = -190.0;
const MAXANGLE: f64 = 190.0;

pub fn check_degree(degree: f64) -> bool {
    MINANGLE <= degree && degree <= MAXANGLE
}

pub fn check_degrees(degrees: &[f64]) -> bool {
    degrees.iter().all(|deg| check_degree(*deg))
}
