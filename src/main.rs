use std::{env, fs};
use std::fs::File;
use std::io::Read;

use crate::ConstantPoolEntry::{InvokeDynamicInfo, MethodHandle, MethodTypeInfo, NameAndTypeInfo, Utf8Info};

fn main() {
    let filename = parse_args();
    println!("Analyzing File {}", filename);

    let data = read_file(&filename);
    println!("size: {} bytes", data.len());
    let mut index: usize = 0;


    let magic = read_u4(&data, &mut index).expect("Expected Magic");
    println!("magic: {:X}", magic);

    let minor = read_u2(&data, &mut index).expect("Expected Minor");
    let major = read_u2(&data, &mut index).expect("Expected Major");
    println!("minor: {}", minor);
    println!("major: {}", major);

    let constant_pool_count = read_u2(&data, &mut index).expect("Expected Constant Pool Count") as usize;
    println!("constant pool count: {}", constant_pool_count);
    let mut constant_pool_entries: Vec<ConstantPoolEntry> = Vec::with_capacity(constant_pool_count);
    for i in 0..constant_pool_count - 1 {
        constant_pool_entries.push(read_constant_pool_entry(&data, &mut index));
        println!("{:02}. {:?}", i + 1, constant_pool_entries.get(i as usize).unwrap());
    }
    let tag = read_u1(&data, &mut index);
    dbg!(tag);
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).expect("Expeced 1 argument but got 0").to_owned()
}

fn read_file(filename: &String) -> Vec<u8> {
    let mut f = File::open(filename).expect("Could not read file");
    let metadata = fs::metadata(filename).expect("Could not read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("Buffer overflow");

    buffer
}

fn read_u1(buffer: &Vec<u8>, index: &mut usize) -> Option<u8> {
    if *index > (buffer.len() - 1) {
        None
    } else {
        let value = *buffer.get(*index).unwrap();
        *index += 1;
        Some(value)
    }
}

fn read_u2(buffer: &Vec<u8>, index: &mut usize) -> Option<u16> {
    if *index > (buffer.len() - 2) {
        None
    } else {
        let value = (*buffer.get(*index).unwrap() as u16) << 8 | (*buffer.get(*index + 1).unwrap() as u16);
        *index += 2;
        Some(value)
    }
}

fn read_u4(buffer: &Vec<u8>, index: &mut usize) -> Option<u32> {
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

fn read_u8(buffer: &Vec<u8>, index: &mut usize) -> Option<u64> {
    let high: u64 = read_u4(buffer, index).expect("Expected Integer") as u64;
    let low: u64 = read_u4(buffer, index).expect("Expected Integer") as u64;
    Some((high << 32) | low)
}

fn read_f4(buffer: &Vec<u8>, index: &mut usize) -> Option<f32> {
    let int = read_u4(buffer, index).expect("Expected Integer");
    Some(f32::from_bits(int))
}

fn read_f8(buffer: &Vec<u8>, index: &mut usize) -> Option<f64> {
    let int = read_u8(buffer, index).expect("Expected Long");
    Some(f64::from_bits(int))
}

fn read_length_and_utf8(buffer: &Vec<u8>, index: &mut usize) -> Option<String> {
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


fn read_constant_pool_entry(buffer: &Vec<u8>, index: &mut usize) -> ConstantPoolEntry {
    let tag = read_u1(buffer, index).expect("Expected Constant Pool Tag");
    match tag {
        7 => ConstantPoolEntry::Class(read_u2(buffer, index).expect("Expected Name Index")),

        9 => ConstantPoolEntry::Fieldref(read_u2(buffer, index).expect("Expected Class Index"),
                                         read_u2(buffer, index).expect("Expected Name And Type Index")),

        10 => ConstantPoolEntry::Methodref(read_u2(buffer, index).expect("Expected Class Index"),
                                           read_u2(buffer, index).expect("Expected Name And Type Index")),

        11 => ConstantPoolEntry::InterfaceMethodref(read_u2(buffer, index).expect("Expected Class Index"),
                                                    read_u2(buffer, index).expect("Expected Name And Type Index")),

        8 => ConstantPoolEntry::StringInfo(read_u2(buffer, index).expect("Expected String Index")),

        3 => ConstantPoolEntry::IntegerInfo(read_u4(buffer, index).expect("Expected Integer")),

        4 => ConstantPoolEntry::FloatInfo(read_f4(buffer, index).expect("Expected Float")),

        5 => ConstantPoolEntry::LongInfo(read_u8(buffer, index).expect("Expected Long")),

        6 => ConstantPoolEntry::DoubleInfo(read_f8(buffer, index).expect("Expected Double")),

        12 => NameAndTypeInfo(read_u2(buffer, index).expect("Expected Name Index"),
                              read_u2(buffer, index).expect("Expected Descriptor Index")),

        1 => Utf8Info(read_length_and_utf8(buffer, index).expect("Expected String")),

        15 => MethodHandle(read_u1(buffer, index).expect("Expected Reference Kind"),
                           read_u2(buffer, index).expect("Expected Reference Index")),

        16 => MethodTypeInfo(read_u2(buffer, index).expect("Expected Descriptor Index")),

        18 => InvokeDynamicInfo(read_u2(buffer, index).expect("Expected Bootstrap Method Attr Index"),
                                read_u2(buffer, index).expect("Expected Name And Type Index")),

        _ => panic!("Invalid Constant Pool Tag")
    }
}

#[derive(Debug)]
enum ConstantPoolEntry {
    Class(u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
    InterfaceMethodref(u16, u16),
    StringInfo(u16),
    IntegerInfo(u32),
    FloatInfo(f32),
    LongInfo(u64),
    DoubleInfo(f64),
    NameAndTypeInfo(u16, u16),
    Utf8Info(String),
    MethodHandle(u8, u16),
    MethodTypeInfo(u16),
    InvokeDynamicInfo(u16, u16),
}
