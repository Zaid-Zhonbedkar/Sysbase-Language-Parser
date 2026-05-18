use pest::iterators::Pair;

use crate::parser::Rule;

use crate::ast::nodes::Program;
use crate::ast::statements::*;
use crate::ast::expressions::*;
pub fn build_program(pair: Pair<Rule>) -> Program {

    let mut statements = Vec::new();

    for inner in pair.into_inner() {

        match inner.as_rule() {

            Rule::statement => {

                let stmt = build_statement(inner);
                statements.push(stmt);
            }

            _ => {}
        }
    }

    Program {
        statements
    }
}
fn build_statement(pair: Pair<Rule>) -> Statement {

    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {

        Rule::variable_decl => build_variable_decl(inner),

        Rule::function_decl => build_function_decl(inner),

        Rule::return_stmt => build_return_stmt(inner),

        Rule::expression_stmt => {
            Statement::ExpressionStmt(
                build_expression(
                    inner.into_inner().next().unwrap()
                )
            )
        }

        _ => {
            panic!("Unhandled statement: {:?}", inner.as_rule())
        }
    }
}
fn build_variable_decl(pair: Pair<Rule>) -> Statement {

    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let next = inner.next().unwrap();

    let value;

    let datatype;

    if next.as_rule() == Rule::type_annotation {

        datatype = Some(
            next.into_inner()
                .next()
                .unwrap()
                .as_str()
                .to_string()
        );

        value = build_expression(
            inner.next().unwrap()
        );

    } else {

        datatype = None;

        value = build_expression(next);
    }

    Statement::VariableDecl {

        name,

        datatype,

        value,

        mutable: true,
    }
}
fn build_expression(pair: Pair<Rule>) -> Expression {

    match pair.as_rule() {

        Rule::integer => {

            Expression::Integer(
                pair.as_str().parse().unwrap()
            )
        }

        Rule::float => {

            Expression::Float(
                pair.as_str().parse().unwrap()
            )
        }

        Rule::string => {

            Expression::String(
                pair.as_str()
                    .trim_matches('"')
                    .to_string()
            )
        }

        Rule::boolean => {

            Expression::Boolean(
                pair.as_str() == "true"
            )
        }

        Rule::identifier => {

            Expression::Identifier(
                pair.as_str().to_string()
            )
        }

        _ => {

            let inner = pair.into_inner().next().unwrap();

            build_expression(inner)
        }
    }
}fn build_return_stmt(pair: Pair<Rule>) -> Statement {

    let expr = pair
        .into_inner()
        .next()
        .map(build_expression);

    Statement::ReturnStmt(expr)
}
fn build_function_decl(pair: Pair<Rule>) -> Statement {

    let mut inner = pair.into_inner();

    let name = inner.next().unwrap()
        .as_str()
        .to_string();

    let mut parameters = Vec::new();

    let mut body = Vec::new();

    for item in inner {

        match item.as_rule() {

            Rule::parameter_list => {

                for param in item.into_inner() {

                    let mut p = param.into_inner();

                    let pname = p.next().unwrap()
                        .as_str()
                        .to_string();

                    let ptype = p.next()
                        .map(|x| x.as_str().to_string());

                    parameters.push(Parameter {

                        name: pname,
                        datatype: ptype,
                    });
                }
            }

            Rule::block => {

                for stmt in item.into_inner() {

                    body.push(
                        build_statement(stmt)
                    );
                }
            }

            _ => {}
        }
    }

    Statement::FunctionDecl {

        name,

        parameters,

        return_type: None,

        body,
    }
}
mod parser;
mod ast;

use pest::Parser;

use parser::NexisParser;
use parser::Rule;

use parser::ast_builder::build_program;

fn main() {

    let source = r#"

        set age: int = 18

        fn greet(name: string) {

            system.out("Hello")

        }

    "#;

    let parsed = NexisParser::parse(
        Rule::program,
        source
    ).unwrap();

    let pair = parsed.into_iter().next().unwrap();

    let ast = build_program(pair);

    println!("{:#?}", ast);
}
