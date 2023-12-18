use std::fs;
use std::fs::File;
use std::io::Read;

use crate::types::{Class, ConstantPool, ConstantPoolEntry};

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


pub fn read_constant_pool_entry(buffer: &Vec<u8>, index: &mut usize) -> ConstantPoolEntry {
    let tag = read_u1(buffer, index).expect("Expected Constant Pool Tag");
    match tag {
        7 => ConstantPoolEntry::Class { name_index: read_u2(buffer, index).expect("Expected Name Index") },

        9 => ConstantPoolEntry::Fieldref {
            class_index: read_u2(buffer, index).expect("Expected Class Index"),
            name_and_type_index: read_u2(buffer, index).expect("Expected Name And Type Index"),
        },

        10 => ConstantPoolEntry::Methodref {
            class_index: read_u2(buffer, index).expect("Expected Class Index"),
            name_and_type_index: read_u2(buffer, index).expect("Expected Name And Type Index"),
        },

        11 => ConstantPoolEntry::InterfaceMethodref {
            class_index: read_u2(buffer, index).expect("Expected Class Index"),
            name_and_type_index: read_u2(buffer, index).expect("Expected Name And Type Index"),
        },

        8 => ConstantPoolEntry::StringInfo { string_index: read_u2(buffer, index).expect("Expected String Index") },

        3 => ConstantPoolEntry::IntegerInfo { value: read_u4(buffer, index).expect("Expected Integer") },

        4 => ConstantPoolEntry::FloatInfo { value: read_f4(buffer, index).expect("Expected Float") },

        5 => ConstantPoolEntry::LongInfo { value: read_u8(buffer, index).expect("Expected Long") },

        6 => ConstantPoolEntry::DoubleInfo { value: read_f8(buffer, index).expect("Expected Double") },

        12 => ConstantPoolEntry::NameAndTypeInfo {
            name_index: read_u2(buffer, index).expect("Expected Name Index"),
            descriptor_index: read_u2(buffer, index).expect("Expected Descriptor Index"),
        },

        1 => ConstantPoolEntry::Utf8Info { value: read_length_and_utf8(buffer, index).expect("Expected String") },

        15 => ConstantPoolEntry::MethodHandle {
            reference_kind: read_u1(buffer, index).expect("Expected Reference Kind"),
            reference_index: read_u2(buffer, index).expect("Expected Reference Index"),
        },

        16 => ConstantPoolEntry::MethodTypeInfo { descriptor_index: read_u2(buffer, index).expect("Expected Descriptor Index") },

        18 => ConstantPoolEntry::InvokeDynamicInfo {
            bootstrap_method_attr_index: read_u2(buffer, index).expect("Expected Bootstrap Method Attr Index"),
            name_and_type_index: read_u2(buffer, index).expect("Expected Name And Type Index"),
        },

        _ => panic!("Invalid Constant Pool Tag")
    }
}

pub fn read_class(buffer: &Vec<u8>, index: &mut usize, constant_pool: &ConstantPool) -> Option<Class> {
    let this_class_index = read_u2(buffer, index).expect("Expected This Class Index") as usize;
    if let ConstantPoolEntry::Class { name_index } = constant_pool.get(this_class_index - 1).unwrap() {
        if let ConstantPoolEntry::Utf8Info { value } = constant_pool.get(*name_index as usize - 1).unwrap() {
            Some(Class {
                name: value.to_owned()
            })
        } else {
            None
        }
    } else {
        None
    }
}