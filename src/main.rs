use std::env;

use crate::io::read_bytes_from_file;
use crate::reader::*;
use crate::types::{Attribute, Class, ConstantPool, Field, FieldFlag, Method, MethodFlag};

mod reader;
mod types;

mod io;


fn main() {
    let filename = parse_args();
    println!("Analyzing File {}", filename);

    let data = read_bytes_from_file(&filename);
    println!("size: {} bytes", data.len());

    let mut constant_pool: ConstantPool = Vec::new();

    let class_file = read_class_file(&data, &mut constant_pool);

    println!("magic: 0x{:X}", class_file.magic);

    println!("Class Version {}.{}", class_file.major_version, class_file.minor_version);

    print_constant_pool(class_file.constant_pool);

    println!("access flags: {:?}", class_file.access_flags);

    println!("class name: {}", class_file.this_class.name);

    println!("super class name: {}", class_file.super_class.name.replace('/', "."));

    for attr in class_file.attributes {
        if let Attribute::SourceFile { source_file } = attr {
            println!("source file: {}", source_file);
        }
    }

    print_interfaces(&class_file.interfaces);

    print_fields(&class_file.fields);

    print_methods(&class_file.methods);
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).expect("Expected 1 argument but got 0").to_owned()
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
        line.push(' ');
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
        } else if method.access_flags.iter().any(|flag| matches!(flag, MethodFlag::AccProtected)) {
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
        line.push(' ');
        line.push_str(method.name.as_str());

        let exception_attr = method.attributes.iter().find(|attr| matches!(attr, Attribute::Exceptions {exceptions: _}));
        if let Some(Attribute::Exceptions { exceptions }) = exception_attr {
            let exceptions: Vec<String> = exceptions.iter().map(|e| e.name.clone()).collect();
            if !exceptions.is_empty() {
                line.push_str(" throws ");
                line.push_str(exceptions.join(", ").as_str())
            }
        }
        println!("{line}");
    }
}

fn print_interfaces(interfaces: &Vec<Class>) {
    println!("implemented interfaces ({}):", interfaces.len());
    interfaces.iter().map(|class| class.name.replace('/', ".")).for_each(|name| {
        println!("  {}", name);
    });
}
