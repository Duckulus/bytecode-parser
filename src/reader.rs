use std::fs;
use std::fs::File;
use std::io::Read;

pub fn read_file(filename: &String) -> Vec<u8> {
    let mut f = File::open(filename).expect("Could not read file");
    let metadata = fs::metadata(filename).expect("Could not read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("Buffer overflow");

    buffer
}

pub fn read_u1(buffer: &Vec<u8>, index: &mut usize) -> Option<u8> {
    if *index > (buffer.len() - 1) {
        None
    } else {
        let value = *buffer.get(*index).unwrap();
        *index += 1;
        Some(value)
    }
}

pub fn read_u2(buffer: &Vec<u8>, index: &mut usize) -> Option<u16> {
    if *index > (buffer.len() - 2) {
        None
    } else {
        let value = (*buffer.get(*index).unwrap() as u16) << 8 | (*buffer.get(*index + 1).unwrap() as u16);
        *index += 2;
        Some(value)
    }
}

pub fn read_u4(buffer: &Vec<u8>, index: &mut usize) -> Option<u32> {
    if *index > (buffer.len() - 4) {
        None
    } else {
        let value = (*buffer.get(*index).unwrap() as u32) << 24 |
            (*buffer.get(*index + 1).unwrap() as u32) << 16 |
            (*buffer.get(*index + 2).unwrap() as u32) << 8 |
            (*buffer.get(*index + 3).unwrap() as u32);
        *index += 4;
        Some(value)
    }
}

pub fn read_u8(buffer: &Vec<u8>, index: &mut usize) -> Option<u64> {
    let high: u64 = read_u4(buffer, index).expect("Expected Integer") as u64;
    let low: u64 = read_u4(buffer, index).expect("Expected Integer") as u64;
    Some((high << 32) | low)
}

pub fn read_f4(buffer: &Vec<u8>, index: &mut usize) -> Option<f32> {
    let int = read_u4(buffer, index).expect("Expected Integer");
    Some(f32::from_bits(int))
}

pub fn read_f8(buffer: &Vec<u8>, index: &mut usize) -> Option<f64> {
    let int = read_u8(buffer, index).expect("Expected Long");
    Some(f64::from_bits(int))
}

pub fn read_length_and_utf8(buffer: &Vec<u8>, index: &mut usize) -> Option<String> {
    if *index > buffer.len() - 1 {
        return None;
    }
    let length = read_u2(buffer, index).expect("Expected Length") as usize;
    if *index + length > buffer.len() {
        return None;
    }
    let string = String::from_utf8(buffer[*index..(*index + length)].to_vec()).unwrap();
    *index += length;
    Some(string)
}


pub fn read_constant_pool_entry(buffer: &Vec<u8>, index: &mut usize) -> crate::ConstantPoolEntry {
    let tag = read_u1(buffer, index).expect("Expected Constant Pool Tag");
    match tag {
        7 => crate::ConstantPoolEntry::Class(read_u2(buffer, index).expect("Expected Name Index")),

        9 => crate::ConstantPoolEntry::Fieldref(read_u2(buffer, index).expect("Expected Class Index"),
                                                read_u2(buffer, index).expect("Expected Name And Type Index")),

        10 => crate::ConstantPoolEntry::Methodref(read_u2(buffer, index).expect("Expected Class Index"),
                                                  read_u2(buffer, index).expect("Expected Name And Type Index")),

        11 => crate::ConstantPoolEntry::InterfaceMethodref(read_u2(buffer, index).expect("Expected Class Index"),
                                                           read_u2(buffer, index).expect("Expected Name And Type Index")),

        8 => crate::ConstantPoolEntry::StringInfo(read_u2(buffer, index).expect("Expected String Index")),

        3 => crate::ConstantPoolEntry::IntegerInfo(read_u4(buffer, index).expect("Expected Integer")),

        4 => crate::ConstantPoolEntry::FloatInfo(read_f4(buffer, index).expect("Expected Float")),

        5 => crate::ConstantPoolEntry::LongInfo(read_u8(buffer, index).expect("Expected Long")),

        6 => crate::ConstantPoolEntry::DoubleInfo(read_f8(buffer, index).expect("Expected Double")),

        12 => crate::ConstantPoolEntry::NameAndTypeInfo(read_u2(buffer, index).expect("Expected Name Index"),
                                                        read_u2(buffer, index).expect("Expected Descriptor Index")),

        1 => crate::ConstantPoolEntry::Utf8Info(read_length_and_utf8(buffer, index).expect("Expected String")),

        15 => crate::ConstantPoolEntry::MethodHandle(read_u1(buffer, index).expect("Expected Reference Kind"),
                                                     read_u2(buffer, index).expect("Expected Reference Index")),

        16 => crate::ConstantPoolEntry::MethodTypeInfo(read_u2(buffer, index).expect("Expected Descriptor Index")),

        18 => crate::ConstantPoolEntry::InvokeDynamicInfo(read_u2(buffer, index).expect("Expected Bootstrap Method Attr Index"),
                                                          read_u2(buffer, index).expect("Expected Name And Type Index")),

        _ => panic!("Invalid Constant Pool Tag")
    }
}