use std::fs;
use std::fs::File;
use std::io::Read;

use crate::types::{AccessFlag, Annotation, Attribute, Class, ConstantPool, ConstantPoolEntry, ElementValue, ElementValuePair, ExceptionHandler, Field, FieldFlag, LineNumber, Method, MethodFlag};

pub fn read_file(filename: &String) -> Vec<u8> {
    let mut f = File::open(filename).expect("Could not read file");
    let metadata = fs::metadata(filename).expect("Could not read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("Buffer overflow");

    buffer
}

pub fn read_constant_pool(buffer: &Vec<u8>, index: &mut usize) -> Vec<ConstantPoolEntry> {
    let constant_pool_count = read_u2(buffer, index).expect("Expected Constant Pool Count") as usize;
    let mut constant_pool: ConstantPool = Vec::with_capacity(constant_pool_count - 1);


    let mut should_put_empty = false;
    for _i in 0..constant_pool_count - 1 {
        if should_put_empty {
            constant_pool.push(ConstantPoolEntry::Empty);
            should_put_empty = false;
        } else {
            let entry = read_constant_pool_entry(buffer, index);
            if matches!(entry, ConstantPoolEntry::DoubleInfo{value: _}) || matches!(entry, ConstantPoolEntry::LongInfo{value: _}) {
                should_put_empty = true;
            }

            constant_pool.push(entry);
        }
    }
    constant_pool
}

pub fn read_interfaces(buffer: &Vec<u8>, index: &mut usize, constant_pool: &ConstantPool) -> Vec<Class> {
    let interfaces_count = read_u2(buffer, index).expect("Expected Interface Count") as usize;
    let mut interfaces: Vec<Class> = Vec::with_capacity(interfaces_count);
    for _ in 0..interfaces_count {
        let index = read_u2(buffer, index).expect("Expected Interface Index") as usize;
        if let ConstantPoolEntry::Class { name_index } = constant_pool.get(index - 1).expect("Expected Class but went out of bounds") {
            if let ConstantPoolEntry::Utf8Info { value } = constant_pool.get(*name_index as usize - 1).expect("Exptected Utf8 but went out of bounds") {
                interfaces.push(Class { name: value.to_owned() })
            } else {
                panic!("Expected Class Name");
            }
        } else {
            panic!("Expected Class Info");
        }
    }
    interfaces
}

pub fn read_fields<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<Field<'a>> {
    let fields_count = read_u2(buffer, index).expect("Expected Fields Count") as usize;
    let mut fields: Vec<Field> = Vec::with_capacity(fields_count);
    for _ in 0..fields_count {
        let flag_mask = read_u2(buffer, index).expect("Expected Element Flags");
        let access_flags = parse_field_flags(flag_mask);

        let name_index = read_u2(buffer, index).expect("Expected Name Index");
        let name = read_utf8_from_constant_pool(constant_pool, name_index).expect("Expected Utf8");

        let descriptor_index = read_u2(buffer, index).expect("Expected Descriptor Index");
        let descriptor = read_utf8_from_constant_pool(constant_pool, descriptor_index).expect("Expected Utf8");

        let attributes = read_attributes(buffer, index, constant_pool);

        fields.push(Field {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
    fields
}

pub fn read_utf8_from_constant_pool(constant_pool: &ConstantPool, index: u16) -> Option<String> {
    if let ConstantPoolEntry::Utf8Info { value } = constant_pool.get(index as usize - 1).expect("Expected Utf8 but went out of bounds") {
        Some(value.to_owned())
    } else {
        None
    }
}

pub fn parse_field_flags(mask: u16) -> Vec<FieldFlag> {
    let mut flags: Vec<FieldFlag> = Vec::new();
    if mask & 0x0001 != 0 {
        flags.push(FieldFlag::AccPublic)
    }
    if mask & 0x0002 != 0 {
        flags.push(FieldFlag::AccPrivate)
    }
    if mask & 0x0004 != 0 {
        flags.push(FieldFlag::AccProtected)
    }
    if mask & 0x0008 != 0 {
        flags.push(FieldFlag::AccStatic)
    }
    if mask & 0x0010 != 0 {
        flags.push(FieldFlag::AccFinal)
    }
    if mask & 0x0040 != 0 {
        flags.push(FieldFlag::AccVolatile)
    }
    if mask & 0x0080 != 0 {
        flags.push(FieldFlag::AccTransient)
    }
    if mask & 0x1000 != 0 {
        flags.push(FieldFlag::AccSynthetic)
    }
    if mask & 0x4000 != 0 {
        flags.push(FieldFlag::AccEnum)
    }
    flags
}

pub fn read_methods<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<Method<'a>> {
    let methods_count = read_u2(buffer, index).expect("Expected Method Count") as usize;
    let mut methods: Vec<Method> = Vec::with_capacity(methods_count);

    for _ in 0..methods_count {
        let flag_mask = read_u2(buffer, index).expect("Expected Acces Flags");
        let access_flags = parse_method_flags(flag_mask);

        let name_index = read_u2(buffer, index).expect("Expected Name Index");
        let name = read_utf8_from_constant_pool(constant_pool, name_index).expect("Expected Utf8");

        let descriptor_index = read_u2(buffer, index).expect("Expected Descriptor Index");
        let descriptor = read_utf8_from_constant_pool(constant_pool, descriptor_index).expect("Expected Utf8");

        let attributes = read_attributes(buffer, index, constant_pool);

        methods.push(Method {
            name,
            descriptor,
            attributes,
            access_flags,
        })
    }

    methods
}

pub fn parse_method_flags(mask: u16) -> Vec<MethodFlag> {
    let mut flags: Vec<MethodFlag> = Vec::new();
    if mask & 0x0001 != 0 {
        flags.push(MethodFlag::AccPublic)
    }
    if mask & 0x0002 != 0 {
        flags.push(MethodFlag::AccPrivate)
    }
    if mask & 0x0004 != 0 {
        flags.push(MethodFlag::AccProtected)
    }
    if mask & 0x0008 != 0 {
        flags.push(MethodFlag::AccStatic)
    }
    if mask & 0x0010 != 0 {
        flags.push(MethodFlag::AccFinal)
    }
    if mask & 0x0020 != 0 {
        flags.push(MethodFlag::AccSynchronized)
    }
    if mask & 0x0040 != 0 {
        flags.push(MethodFlag::AccBridge)
    }
    if mask & 0x0080 != 0 {
        flags.push(MethodFlag::AccVarargs)
    }
    if mask & 0x0100 != 0 {
        flags.push(MethodFlag::AccNative)
    }
    if mask & 0x0400 != 0 {
        flags.push(MethodFlag::AccAbstract)
    }
    if mask & 0x0800 != 0 {
        flags.push(MethodFlag::AccStrict)
    }
    if mask & 0x1000 != 0 {
        flags.push(MethodFlag::AccSynthetic)
    }
    flags
}

pub fn read_attributes<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<Attribute<'a>> {
    let attributes_count = read_u2(buffer, index).expect("Expected Attribute Count") as usize;
    let mut attributes: Vec<Attribute> = Vec::with_capacity(attributes_count);
    for _ in 0..attributes_count {
        let name_index = read_u2(buffer, index).expect("Expected Attribute Name Index");
        let name = read_utf8_from_constant_pool(constant_pool, name_index).expect("Expected Utf8");
        read_u4(buffer, index).expect("Expected Attribute length");

        let attribute = match name.as_str() {
            "ConstantValue" => {
                let constantvalue_index = read_u2(buffer, index).expect("Expected Constant Value Index") as usize;
                Attribute::ConstantValue { value: constant_pool.get(constantvalue_index - 1).expect("Expected Constant Value") }
            }

            "Synthetic" => Attribute::Synthetic,

            "Signature" => {
                let signature_index = read_u2(buffer, index).expect("Expected Signature Index");
                let signature = read_utf8_from_constant_pool(constant_pool, signature_index).expect("Expected Utf8");
                Attribute::Signature { signature }
            }

            "Deprecated" => Attribute::Deprecated,

            "RuntimeVisibleAnnotations" => {
                Attribute::RuntimeVisibleAnnotations { annotations: read_annotations(buffer, index, constant_pool) }
            }

            "RuntimeInvisibleAnnotations" => {
                Attribute::RuntimeInvisibleAnnotations { annotations: read_annotations(buffer, index, constant_pool) }
            }

            "Code" => {
                let max_stack = read_u2(buffer, index).expect("Expected Max Stack");
                let max_locals = read_u2(buffer, index).expect("Expected Max Locals");

                let code_length = read_u4(buffer, index).expect("Expected Code Length");
                let mut code: Vec<u8> = Vec::new();
                for _ in 0..code_length {
                    code.push(read_u1(buffer, index).expect("Expected Code"))
                }
                let exception_table = read_exception_table(buffer, index, constant_pool);
                let attributes = read_attributes(buffer, index, constant_pool);

                Attribute::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }
            }

            "Exceptions" => {
                Attribute::Exceptions { exceptions: read_exceptions(buffer, index, constant_pool) }
            }

            "RuntimeVisibleParameterAnnotations" => {
                Attribute::RuntimeVisibleParameterAnnotations { annotations: read_parameter_annotations(buffer, index, constant_pool) }
            }

            "RuntimeInvisibleParameterAnnotations" => {
                Attribute::RuntimeInvisibleParameterAnnotations { annotations: read_parameter_annotations(buffer, index, constant_pool) }
            }

            "AnnotationDefault" => {
                Attribute::AnnotationDefault { default_value: read_element_value(buffer, index, constant_pool) }
            }

            "LineNumberTable" => {
                Attribute::LineNumberTable { line_number_table: read_line_number_table(buffer, index) }
            }

            _ => panic!("Invalid Attribute Name: {}", name)
        };

        attributes.push(attribute);
    }
    attributes
}

pub fn read_exception_table(buffer: &Vec<u8>, index: &mut usize, constant_pool: &ConstantPool) -> Vec<ExceptionHandler> {
    let exception_table_length = read_u2(buffer, index).expect("Expected Exception Table Length");
    let mut exception_table: Vec<ExceptionHandler> = Vec::new();
    for _ in 0..exception_table_length {
        let start_pc = read_u2(buffer, index).expect("Expected Start PC");
        let end_pc = read_u2(buffer, index).expect("Expected End PC");
        let handler_pc = read_u2(buffer, index).expect("Expected Handler PC");
        let catch_type_index = read_u2(buffer, index).expect("Expected Catch Type") as usize;
        let catch_type: Option<Class>;
        if catch_type_index == 0 {
            catch_type = None;
        } else {
            if let ConstantPoolEntry::Class { name_index } = constant_pool.get(catch_type_index - 1).expect("Expected Constant Pool Entry") {
                let class_name = read_utf8_from_constant_pool(constant_pool, *name_index).expect("Expected Utf8");
                catch_type = Some(Class { name: class_name })
            } else {
                panic!("Catch Type must be a Class")
            }
        };

        exception_table.push(ExceptionHandler {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
    exception_table
}

pub fn read_exceptions(buffer: &Vec<u8>, index: &mut usize, constant_pool: &ConstantPool) -> Vec<Class> {
    let exceptions_number = read_u2(buffer, index).expect("Expected Expeption Number") as usize;
    let mut exceptions: Vec<Class> = Vec::with_capacity(exceptions_number);

    for _ in 0..exceptions_number {
        let exception_index = read_u2(buffer, index).expect("Expected Exception Index") as usize;
        if let ConstantPoolEntry::Class { name_index } = constant_pool.get(exception_index - 1).expect("Expected Constant Pool Entry") {
            let class_name = read_utf8_from_constant_pool(constant_pool, *name_index).expect("Expected Class");
            exceptions.push(Class { name: class_name })
        } else {
            panic!("Exception must be a Class")
        }
    }

    exceptions
}

pub fn read_line_number_table(buffer: &Vec<u8>, index: &mut usize) -> Vec<LineNumber> {
    let line_number_count = read_u2(buffer, index).expect("Expected Line Number Count") as usize;
    let mut line_numbers: Vec<LineNumber> = Vec::with_capacity(line_number_count);

    for _ in 0..line_number_count {
        let start_pc = read_u2(buffer, index).expect("Expected Start PC");
        let line_number = read_u2(buffer, index).expect("Expected Line Number");
        line_numbers.push(LineNumber { start_pc, line_number })
    }

    line_numbers
}

pub fn read_parameter_annotations<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<Vec<Annotation<'a>>> {
    let num_parameters = read_u1(buffer, index).expect("Expected Parameter Number") as usize;
    let mut parameter_annotations: Vec<Vec<Annotation>> = Vec::with_capacity(num_parameters);

    for _ in 0..num_parameters {
        parameter_annotations.push(read_annotations(buffer, index, constant_pool));
    }

    parameter_annotations
}

pub fn read_annotations<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<Annotation<'a>> {
    let annotations_count = read_u2(buffer, index).expect("Expected Annotation Count") as usize;
    let mut annotations: Vec<Annotation> = Vec::with_capacity(annotations_count);

    for _ in 0..annotations_count {
        annotations.push(read_annotation(buffer, index, constant_pool));
    }

    annotations
}

pub fn read_annotation<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Annotation<'a> {
    let type_index = read_u2(buffer, index).expect("Expected Type Index");
    let type_name = read_utf8_from_constant_pool(constant_pool, type_index).expect("Expected Utf8");

    let element_value_pairs = read_element_value_pairs(buffer, index, constant_pool);

    Annotation {
        type_name,
        element_value_pairs,
    }
}

pub fn read_element_value_pairs<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> Vec<ElementValuePair<'a>> {
    let pair_count = read_u2(buffer, index).expect("Expected Pair Count") as usize;
    let mut pairs: Vec<ElementValuePair> = Vec::with_capacity(pair_count);

    for _ in 0..pair_count {
        let element_name_index = read_u2(buffer, index).expect("Expected Element Name Index");
        let element_name = read_utf8_from_constant_pool(constant_pool, element_name_index).expect("Expected Utf8");
        let element_value = read_element_value(buffer, index, constant_pool);
        pairs.push(ElementValuePair(element_name, element_value));
    }

    pairs
}

pub fn read_element_value<'a>(buffer: &Vec<u8>, index: &mut usize, constant_pool: &'a ConstantPool) -> ElementValue<'a> {
    let tag = read_u1(buffer, index).expect("Expected Tag") as char;

    match tag {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
            let const_value_index = read_u2(buffer, index).expect("Expected Const Value Index") as usize;
            ElementValue::ConstValue { value: constant_pool.get(const_value_index - 1).expect("Expected Constant Pool Entry") }
        }

        'e' => {
            let type_name_index = read_u2(buffer, index).expect("Expected Type Name Index");
            let type_name = read_utf8_from_constant_pool(constant_pool, type_name_index).expect("Expected Utf8");

            let const_name_index = read_u2(buffer, index).expect("Expected Const Name Index");
            let const_name = read_utf8_from_constant_pool(constant_pool, const_name_index).expect("Expected Utf8");
            ElementValue::EnumConstValue { type_name, const_name }
        }

        'c' => {
            let class_info_index = read_u2(buffer, index).expect("Expected Class Info Index");
            let class_info = read_utf8_from_constant_pool(constant_pool, class_info_index).expect("Expected Utf8");
            ElementValue::ClassInfo { descriptor: class_info }
        }

        '@' => ElementValue::AnnotationValue { annotation: read_annotation(buffer, index, constant_pool) },

        '[' => {
            let num_values = read_u2(buffer, index).expect("Expected Array Value Count") as usize;
            let mut elements: Vec<ElementValue> = Vec::with_capacity(num_values);

            for _ in 0..num_values {
                elements.push(read_element_value(buffer, index, constant_pool));
            }
            ElementValue::ArrayValue { elements }
        }
        _ => panic!("Invalid Element Value Tag: {}", tag)
    }
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

pub fn read_access_flags(buffer: &Vec<u8>, index: &mut usize) -> Vec<AccessFlag> {
    let access_flags_mask = read_u2(buffer, index).expect("Expected Access Flags");
    return parse_access_flags(access_flags_mask);
}

fn parse_access_flags(mask: u16) -> Vec<AccessFlag> {
    let mut flags: Vec<AccessFlag> = Vec::new();
    if mask & 0x0001 != 0 {
        flags.push(AccessFlag::AccPublic);
    }
    if mask & 0x0010 != 0 {
        flags.push(AccessFlag::AccFinal);
    }
    if mask & 0x0020 != 0 {
        flags.push(AccessFlag::AccSuper);
    }
    if mask & 0x0200 != 0 {
        flags.push(AccessFlag::AccInterface);
    }
    if mask & 0x0400 != 0 {
        flags.push(AccessFlag::AccAbstract);
    }
    if mask & 0x1000 != 0 {
        flags.push(AccessFlag::AccSynthetic);
    }
    if mask & 0x2000 != 0 {
        flags.push(AccessFlag::AccAnnotation);
    }
    if mask & 0x4000 != 0 {
        flags.push(AccessFlag::AccEnum);
    }
    flags
}