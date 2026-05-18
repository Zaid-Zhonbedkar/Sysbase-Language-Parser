use crate::ast::types::NexisType;

pub fn to_rust_type(
    datatype: &NexisType
) -> String {

    match datatype {

        NexisType::Int =>
            "i64".into(),

        NexisType::Float =>
            "f64".into(),

        NexisType::Bool =>
            "bool".into(),

        NexisType::String =>
            "String".into(),

        NexisType::Char =>
            "char".into(),

        NexisType::Void =>
            "()".into(),

        NexisType::List =>
            "Vec<()>".into(),

        NexisType::Tensor =>
            "Tensor".into(),

        NexisType::Memory =>
            "Memory".into(),

        NexisType::Agent =>
            "Agent".into(),

        NexisType::Map =>
            "HashMap<(), ()>".into(),

        NexisType::Buffer =>
            "Vec<u8>".into(),

        NexisType::Custom(name) =>
            name.clone(),
    }
}