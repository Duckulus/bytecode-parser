pub type ConstantPool = Vec<ConstantPoolEntry>;

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

#[derive(Debug)]
pub struct Field<'a> {
    pub access_flags: Vec<String>,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug)]
pub enum Attribute<'a> {
    ConstantValue { value: &'a ConstantPoolEntry },
    Synthetic,
    Signature { signature: String },
    Deprecated,
    RuntimeVisibleAnnotations { annotations: Vec<Annotation<'a>> },
    RuntimeInvisibleAnnotations { annotations: Vec<Annotation<'a>> },
}

#[derive(Debug)]
pub struct Annotation<'a> {
    pub type_name: String,
    pub element_value_pairs: Vec<ElementValuePair<'a>>,
}

#[derive(Debug)]
pub struct ElementValuePair<'a>(pub String, pub ElementValue<'a>);

#[derive(Debug)]
pub enum ElementValue<'a> {
    ConstValue { value: &'a ConstantPoolEntry },
    EnumConstValue { type_name: String, const_name: String },
    ClassInfo { descriptor: String },
    AnnotationValue { annotation: Annotation<'a> },
    ArrayValue { elements: Vec<ElementValue<'a>> },
}
