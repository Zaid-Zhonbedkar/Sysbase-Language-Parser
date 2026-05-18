use pest::Parser;
use pest::iterators::Pair;

use pest_derive::Parser;

use crate::ast::{
    Program,
    Statement,
    Expression,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct NexisParser;

pub fn build_ast(
    pair: Pair<Rule>
) -> Program {

    let mut statements = Vec::new();

    for inner in pair.into_inner() {

        if inner.as_rule() == Rule::statement {

            let stmt =
                build_statement(inner);

            statements.push(stmt);
        }
    }

    Program {
        statements
    }
}

fn build_statement(
    pair: Pair<Rule>
) -> Statement {

    let inner =
        pair.into_inner()
            .next()
            .unwrap();

    match inner.as_rule() {

        Rule::variable_decl => {

            let mut parts =
                inner.into_inner();

            let name =
                parts.next()
                    .unwrap()
                    .as_str()
                    .to_string();

            let value =
                build_expression(
                    parts.next().unwrap()
                );

            Statement::VariableDecl {

                name,

                value,
            }
        }

        Rule::print_stmt => {

            let expr =
                build_expression(
                    inner.into_inner()
                        .next()
                        .unwrap()
                );

            Statement::PrintStmt(expr)
        }

        _ => panic!("Unknown statement"),
    }
}

fn build_expression(
    pair: Pair<Rule>
) -> Expression {

    match pair.as_rule() {

        Rule::integer => {

            Expression::Integer(
                pair.as_str()
                    .parse()
                    .unwrap()
            )
        }

        Rule::string => {

            Expression::String(

                pair.as_str()

                    .trim_matches('"')

                    .to_string()
            )
        }

        Rule::identifier => {

            Expression::Identifier(
                pair.as_str()
                    .to_string()
            )
        }

        Rule::binary_expr => {

            let mut inner =
                pair.into_inner();

            let left =
                build_expression(
                    inner.next().unwrap()
                );

            let operator =
                inner.next()
                    .unwrap()
                    .as_str()
                    .to_string();

            let right =
                build_expression(
                    inner.next().unwrap()
                );

            Expression::Binary {

                left: Box::new(left),

                operator,

                right: Box::new(right),
            }
        }

        _ => {

            let inner =
                pair.into_inner()
                    .next()
                    .unwrap();

            build_expression(inner)
        }
    }
}