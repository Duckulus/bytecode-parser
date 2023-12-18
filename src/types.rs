#[derive(Debug)]
pub enum ConstantPoolEntry {
    Class { name_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    StringInfo { string_index: u16 },
    IntegerInfo { value: u32 },
    FloatInfo { value: f32 },
    LongInfo { value: u64 },
    DoubleInfo { value: f64 },
    NameAndTypeInfo { name_index: u16, descriptor_index: u16 },
    Utf8Info { value: String },
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodTypeInfo { descriptor_index: u16 },
    InvokeDynamicInfo { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    Empty, // Used to represent the empty space after a Double or a Long
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
}

pub type ConstantPool = Vec<ConstantPoolEntry>;