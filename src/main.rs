use std::env;

use crate::reader::*;

mod reader;
mod types;


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

    let constant_pool = read_constant_pool(&data, &mut index);
    println!("constant pool count: {}", constant_pool.len() + 1);

    let access_flags_mask = read_u2(&data, &mut index).expect("Expected Access Flags");
    println!("access flags: {}", parse_access_flags(access_flags_mask).join(", "));

    let this_class = read_class(&data, &mut index, &constant_pool).expect("Expected This Class");
    println!("class name: {}", this_class.name);

    let super_class = read_class(&data, &mut index, &constant_pool).expect("Expected Super Class");
    println!("super class name: {}", super_class.name);

    let interfaces = read_interfaces(&data, &mut index, &constant_pool);
    let interface_names: Vec<String> = interfaces.iter().map(|class| class.name.clone()).collect();
    println!("implements {} interfaces: {{{}}}", interfaces.len(), interface_names.join(", "))
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).expect("Expeced 1 argument but got 0").to_owned()
}

fn parse_access_flags(mask: u16) -> Vec<&'static str> {
    let mut flags: Vec<&str> = Vec::new();
    if mask & 0x0001 != 0 {
        flags.push("public");
    }
    if mask & 0x0010 != 0 {
        flags.push("final");
    }
    if mask & 0x0020 != 0 {
        flags.push("super");
    }
    if mask & 0x0200 != 0 {
        flags.push("interface");
    }
    if mask & 0x0400 != 0 {
        flags.push("abstract");
    }
    if mask & 0x1000 != 0 {
        flags.push("synthetic");
    }
    if mask & 0x2000 != 0 {
        flags.push("annotation");
    }
    if mask & 0x4000 != 0 {
        flags.push("enum");
    }
    flags
}




