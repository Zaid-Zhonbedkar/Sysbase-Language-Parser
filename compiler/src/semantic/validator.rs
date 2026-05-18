use crate::ast::expressions::Expression;
use crate::ast::types::NexisType;

pub fn infer_expression_type(
    expr: &Expression
) -> NexisType {

    match expr {

        Expression::Integer(_) => NexisType::Int,

        Expression::Float(_) => NexisType::Float,

        Expression::String(_) => NexisType::String,

        Expression::Boolean(_) => NexisType::Bool,

        _ => NexisType::Void,
    }
}