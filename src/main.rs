use std::env;

use crate::reader::*;
use crate::types::{Attribute, ConstantPool, Field, FieldFlag, Method, MethodFlag};

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
    println!("Class Version {major}.{minor}");

    let constant_pool = read_constant_pool(&data, &mut index);
    print_constant_pool(&constant_pool);

    let access_flags = read_access_flags(&data, &mut index);
    println!("access flags: {:?}", access_flags);

    let this_class = read_class(&data, &mut index, &constant_pool).expect("Expected This Class");
    println!("class name: {}", this_class.name);

    let super_class = read_class(&data, &mut index, &constant_pool).expect("Expected Super Class");
    println!("super class name: {}", super_class.name);

    let interfaces = read_interfaces(&data, &mut index, &constant_pool);
    let interface_names: Vec<String> = interfaces.iter().map(|class| class.name.clone()).collect();
    println!("implemented interfaces ({}): {{{}}}", interfaces.len(), interface_names.join(", "));

    let fields = read_fields(&data, &mut index, &constant_pool);
    print_fields(&fields);

    let methods = read_methods(&data, &mut index, &constant_pool);
    print_methods(&methods);

    let attributes = read_attributes(&data, &mut index, &constant_pool);

    println!("Parsed {} bytes", index);
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).expect("Expeced 1 argument but got 0").to_owned()
}

fn print_constant_pool(constant_pool: &ConstantPool) {
    println!("constant pool ({}):", constant_pool.len() + 1);
    for (i, entry) in constant_pool.iter().enumerate() {
        println!("  #{:02} {:?}", i + 1, entry);
    }
}

fn print_fields(fields: &Vec<Field>) {
    println!("fields ({}):", fields.len());
    for field in fields {
        let mut line = String::from("  ");
        if field.access_flags.iter().any(|flag| matches!(flag, FieldFlag::AccPublic)) {
            line.push_str("public ")
        } else if field.access_flags.iter().any(|flag| matches!(flag, FieldFlag::AccPrivate)) {
            line.push_str("private ")
        } else if field.access_flags.iter().any(|flag| matches!(flag, FieldFlag::AccProtected)) {
            line.push_str("public ")
        }
        if field.access_flags.iter().any(|flag| matches!(flag, FieldFlag::AccStatic)) {
            line.push_str("static ")
        }
        if field.access_flags.iter().any(|flag| matches!(flag, FieldFlag::AccFinal)) {
            line.push_str("final ")
        }

        line.push_str(field.type_name().as_str());
        line.push_str(" ");
        line.push_str(field.name.as_str());

        let constant_value_attr = field.attributes.iter().find(|attr| matches!(attr, Attribute::ConstantValue {value: _}));
        if let Some(Attribute::ConstantValue { value }) = constant_value_attr {
            let const_value = value.const_value_as_string();
            if let Some(value) = const_value {
                line.push_str(" = ");
                if field.descriptor == "Z" {
                    line.push_str(if value == "1" { "true" } else { "false" })
                } else {
                    line.push_str(value.as_str());
                }
            }
        }

        println!("{line}")
    }
}

fn print_methods(methods: &Vec<Method>) {
    println!("methods ({}):", methods.len());
    for method in methods {
        let mut line = String::from("  ");
        if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccPublic)) {
            line.push_str("public ")
        } else if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccPrivate)) {
            line.push_str("private ")
        }
        else if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccProtected)) {
            line.push_str("protected ")
        }
        if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccAbstract)) {
            line.push_str("abstract ")
        }
        if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccStatic)) {
            line.push_str("static ")
        }
        if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccFinal)) {
            line.push_str("final ")
        }
        if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccSynchronized)) {
            line.push_str("synchronized ")
        }
        line.push_str(method.descriptor.as_str());
        line.push_str(" ");
        line.push_str(method.name.as_str());

        let exception_attr = method.attributes.iter().find(|attr| matches!(attr, Attribute::Exceptions {exceptions: _}));
        if let Some(Attribute::Exceptions {exceptions}) = exception_attr {
            let exceptions: Vec<String> = exceptions.iter().map(|e| e.name.clone()).collect();
            if !exceptions.is_empty() {
                line.push_str(" throws ");
                line.push_str(exceptions.join(", ").as_str())
            }
        }
        println!("{line}");
    }
}





