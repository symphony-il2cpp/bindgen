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
    String,
    Object,
    Unknown,
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
            "string" | "String" | "System.String" => Self::String,
            "object" | "Object" | "System.Object" => Self::Object,
            _ => Self::Unknown,
        }
    }
}
