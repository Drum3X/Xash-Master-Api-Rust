
pub fn unpack_u8(data: &mut Vec<u8>) -> u8 {
    let (value_bytes, rest) = data.split_at(1);
    let value = value_bytes[0];
    
    *data = rest.to_vec();
    value
}

pub fn unpack_i16(data: &mut Vec<u8>) -> i16 {
    let (value_bytes, rest) = data.split_at(2);
    let value = i16::from_le_bytes([value_bytes[0], value_bytes[1]]);
    
    *data = rest.to_vec();
    value 
}

//for port parsing 
pub fn unpack_u16_be(data: &mut Vec<u8>) -> u16 {
    let (value_bytes, rest) = data.split_at(2);
    let value = u16::from_be_bytes([value_bytes[0], value_bytes[1]]);
    
    *data = rest.to_vec();
    value
}

pub fn unpack_i32(data: &mut Vec<u8>) -> i32 {
    let (value_bytes, rest) = data.split_at(4);
    let value = i32::from_le_bytes([
        value_bytes[0],
        value_bytes[1],
        value_bytes[2],
        value_bytes[3],
    ]);
    
    *data = rest.to_vec();
    value
}

pub fn unpack_string(data: &mut Vec<u8>) -> String {
    let index = match data.iter().position(|&x| x == 0) {
        Some(index) => index,
        None => 0,
    };
    
    let ref_data = data.clone();
    let (value, rest) = ref_data.split_at(index + 1);
    
    *data = rest.to_vec();
    String::from_utf8(value[..value.len() - 1].to_vec()).unwrap()
}
