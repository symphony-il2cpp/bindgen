use quote::{format_ident, quote};
use std::{collections::HashSet, convert::Infallible, fmt, str::FromStr};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    Usize,
    String,
    Object,
    Unit,
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Variable {
    t: Type,
    name: String,
}

type Args = Vec<Variable>;
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Method {
    t: Type,
    name: String,
    this: bool,
    args: Args,
}

type Namespace = Vec<String>;
type Fields = HashSet<Variable>;
type Methods = HashSet<Method>;
#[derive(Clone, Debug)]
struct Class {
    namespace: Namespace,
    name: String,
    fields: Fields,
    methods: Methods,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "sbyte" | "SByte" | "System.SByte" => Self::I8,
            "byte" | "Byte" | "System.Byte" | "bool" | "Boolean" | "System.Boolean" => Self::U8,
            "short" | "Int16" | "System.Int16" => Self::I16,
            "ushort" | "UInt16" | "System.UInt16" | "char" | "Char" | "System.Char" => Self::U16,
            "int" | "Int32" | "System.Int32" => Self::I32,
            "uint" | "UInt32" | "System.UInt32" => Self::U32,
            "long" | "Int64" | "System.Int64" => Self::I64,
            "ulong" | "UInt64" | "System.UInt64" => Self::U64,
            "float" | "Float" | "System.Float" => Self::F32,
            "double" | "Double" | "System.Double" => Self::F64,
            "IntPtr" | "System.IntPtr" => Self::ISize,
            "UintPtr" | "System.UIntPtr" => Self::Usize,
            "string" | "String" | "System.String" => Self::String,
            "object" | "Object" | "System.Object" => Self::Object,
            "void" | "Void" | "System.Void" => Self::Unit,
            _ => Self::Unknown,
        }
    }
}

impl FromStr for Type {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Into<&'static str> for Type {
    fn into(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::U8 => "u8",
            Self::I16 => "i16",
            Self::U16 => "u16",
            Self::I32 => "i32",
            Self::U32 => "u32",
            Self::I64 => "i64",
            Self::U64 => "u64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::ISize => "isize",
            Self::Usize => "usize",
            Self::String => "*mut symphony_il2cpp::types::Il2CppString",
            Self::Object => "*mut symphony_il2cpp::types::Il2CppObject",
            Self::Unit => "()",
            Self::Unknown => "*mut std::ffi::c_void",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.name, self.t)
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let namespace = self.namespace.join(".");
        let namespace = format_ident!("{}", namespace);

        let name = &self.name;

        let field_names = self.fields.iter().map(|f| f.name);
        let field_types = self.fields.iter().map(|f| f.t.into());
        let fields = quote! {
            #(pub #field_names: #field_types),*
        };

        let struct_def = quote! {
            #[repr(align(8))]
            #[derive(Copy, Clone, Debug)]
            pub struct fields_#name {
                #fields
            }

            #[derive(Copy, Clone, Debug)]
            pub struct #name {
                pub inner: *mut symphony_il2cpp::types::Il2CppObject,
                pub class: *mut symphony_il2cpp::types::Il2CppClass,
                pub fields: Option<fields_#name>,
            }

            impl std::convert::TryFrom<*mut symphony_il2cpp::types::Il2CppObject> for #name {
                type Error = symphony_il2cpp::error::Error;

                fn try_from(value: *mut symphony_il2cpp::types::Il2CppObject) -> Result<Self, Self::Error> {{
                    let class = symphony_il2cpp::utils::get_class_from_name(#namespace, #name)?;
                    Ok(Self {{
                        inner: value,
                        class,
                        fields: None,
                    }})
                }}
            }
        };
    }
}
