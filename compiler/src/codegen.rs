use crate::ast::{
    Program,
    Statement,
    Expression,
};

pub fn generate_rust(
    program: &Program
) -> String {

    let mut output =
        String::new();

    output.push_str(
        "fn main() {\n"
    );

    for stmt in &program.statements {

        output.push_str(
            &generate_statement(stmt)
        );
    }

    output.push_str("}\n");

    output
}

fn generate_statement(
    stmt: &Statement
) -> String {

    match stmt {

        Statement::VariableDecl {

            name,

            value,

        } => {

            format!(
                "let mut {} = {};\n",

                name,

                generate_expression(value)
            )
        }

        Statement::PrintStmt(expr) => {

            format!(
                "println!(\"{{:?}}\", {});\n",

                generate_expression(expr)
            )
        }
    }
}

fn generate_expression(
    expr: &Expression
) -> String {

    match expr {

        Expression::Integer(v) => {

            v.to_string()
        }

        Expression::String(v) => {

            format!(
                "\"{}\"",
                v
            )
        }

        Expression::Identifier(v) => {

            v.clone()
        }

        Expression::Binary {

            left,

            operator,

            right,

        } => {

            format!(
                "{} {} {}",

                generate_expression(left),

                operator,

                generate_expression(right)
            )
        }
    }
}