use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashSet, convert::Infallible, fmt, str::FromStr};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Type {
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
    Unit,
    String,
    Object,
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Variable {
    pub t: Type,
    pub name: String,
}

pub type Args = Vec<Variable>;
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Method {
    t: Type,
    name: String,
    this: bool,
    args: Args,
}

pub type Namespace = Vec<String>;
pub type Fields = HashSet<Variable>;
pub type Methods = HashSet<Method>;
#[derive(Clone, Debug)]
pub struct Class {
    pub namespace: Namespace,
    pub name: String,
    pub fields: Fields,
    pub methods: Methods,
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
            "void" | "Void" | "System.Void" => Self::Unit,
            "string" | "String" | "System.String" => Self::String,
            "object" | "Object" | "System.Object" => Self::Object,
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

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::I8 => tokens.extend(quote! {i8}),
            Self::U8 => tokens.extend(quote! {u8}),
            Self::I16 => tokens.extend(quote! {i16}),
            Self::U16 => tokens.extend(quote! {u16}),
            Self::I32 => tokens.extend(quote! {i32}),
            Self::U32 => tokens.extend(quote! {u32}),
            Self::I64 => tokens.extend(quote! {i64}),
            Self::U64 => tokens.extend(quote! {u64}),
            Self::F32 => tokens.extend(quote! {f32}),
            Self::F64 => tokens.extend(quote! {f64}),
            Self::ISize => tokens.extend(quote! {isize}),
            Self::Usize => tokens.extend(quote! {usize}),
            Self::Unit => tokens.extend(quote! {()}),
            Self::String => {
                tokens.extend(quote! {*mut symphony_il2cpp::types::Il2CppString});
            }
            Self::Object => tokens.extend(quote! {*mut symphony_il2cpp::types::Il2CppObject}),
            Self::Unknown => tokens.extend(quote! {*mut std::ffi::c_void}),
        }
    }
}

impl ToTokens for Variable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let t = self.t;
        tokens.extend(quote! {#name: #t})
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let namespace_str = self.namespace.join(".");
        let name = format_ident!("{}", self.name);
        let fields_name = format_ident!("fields_{}", self.name);
        let name_str = &self.name;

        let fields = self.fields.iter();
        let fields = quote! {
            #(pub #fields),*
        };

        let struct_def = quote! {
            #[repr(align(8))]
            #[derive(Copy, Clone, Debug)]
            pub struct #fields_name {
                #fields
            }

            #[derive(Copy, Clone, Debug)]
            pub struct #name {
                pub inner: *mut symphony_il2cpp::types::Il2CppObject,
                pub class: *mut symphony_il2cpp::types::Il2CppClass,
                pub fields: Option<#fields_name>,
            }

            impl std::convert::TryFrom<*mut symphony_il2cpp::types::Il2CppObject> for #name {
                type Error = symphony_il2cpp::error::Error;

                fn try_from(value: *mut symphony_il2cpp::types::Il2CppObject) -> Result<Self, Self::Error> {{
                    let class = symphony_il2cpp::utils::get_class_from_name(#namespace_str, #name_str)?;
                    Ok(Self {{
                        inner: value,
                        class,
                        fields: None,
                    }})
                }}
            }
        };

        write!(f, "{}", struct_def)
    }
}
