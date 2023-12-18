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

impl ConstantPoolEntry {
    pub fn const_value_as_string(&self) -> Option<String> {
        match self {
            ConstantPoolEntry::IntegerInfo { value } => Some(value.to_string()),
            ConstantPoolEntry::LongInfo { value } => Some(value.to_string()),
            ConstantPoolEntry::FloatInfo { value } => Some(value.to_string()),
            ConstantPoolEntry::DoubleInfo { value } => Some(value.to_string()),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
}

#[derive(Debug)]
pub enum AccessFlag {
    AccPublic,
    AccFinal,
    AccSuper,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub access_flags: Vec<FieldFlag>,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute<'a>>,
}

impl Field<'_> {
    pub fn type_name(&self) -> String {
        Field::type_name_from_string(&self.descriptor)
    }

    fn type_name_from_string(string: &String) -> String {
        match string.as_str() {
            "B" => String::from("byte"),
            "C" => String::from("char"),
            "D" => String::from("double"),
            "F" => String::from("float"),
            "I" => String::from("int"),
            "J" => String::from("long"),
            "Z" => String::from("boolean"),
            "S" => String::from("short"),
            _ => {
                if string.starts_with("L") {
                    string.clone()[1..string.len() - 1].replace("/", ".")
                } else if string.starts_with("[") {
                    let mut copy = string.clone();
                    while copy.starts_with("[") {
                        copy.remove(0);
                        copy = Field::type_name_from_string(&copy);
                        copy.push_str("[]");
                    }
                    copy
                } else {
                    String::new()
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum FieldFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccVolatile,
    AccTransient,
    AccSynthetic,
    AccEnum,
}

#[derive(Debug)]
pub struct Method<'a> {
    pub access_flags: Vec<MethodFlag>,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug)]
pub enum MethodFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccSynchronized,
    AccBridge,
    AccVarargs,
    AccNative,
    AccAbstract,
    AccStrict,
    AccSynthetic,
}

#[derive(Debug)]
pub enum Attribute<'a> {
    ConstantValue { value: &'a ConstantPoolEntry },
    Synthetic,
    Signature { signature: String },
    Deprecated,
    RuntimeVisibleAnnotations { annotations: Vec<Annotation<'a>> },
    RuntimeInvisibleAnnotations { annotations: Vec<Annotation<'a>> },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionHandler>,
        attributes: Vec<Attribute<'a>>,
    },
    Exceptions { exceptions: Vec<Class> },
    RuntimeVisibleParameterAnnotations { annotations: Vec<Vec<Annotation<'a>>> },
    RuntimeInvisibleParameterAnnotations { annotations: Vec<Vec<Annotation<'a>>> },
    AnnotationDefault { default_value: ElementValue<'a> },
    LineNumberTable {
        line_number_table: Vec<LineNumber>
    },
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<Class>,
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
