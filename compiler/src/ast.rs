#[derive(Debug, Clone)]
pub enum Expression {

    Integer(i64),

    String(String),

    Identifier(String),

    Binary {

        left: Box<Expression>,

        operator: String,

        right: Box<Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum Statement {

    VariableDecl {

        name: String,

        value: Expression,
    },

    PrintStmt(Expression),
}

#[derive(Debug)]
pub struct Program {

    pub statements: Vec<Statement>,
}