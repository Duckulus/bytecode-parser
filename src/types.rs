#[derive(Debug)]
pub enum ConstantPoolEntry {
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

pub struct Class {
    pub name: String,
}

pub type ConstantPool = Vec<ConstantPoolEntry>;