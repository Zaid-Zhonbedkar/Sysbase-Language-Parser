    fn generate_statement(
        &mut self,
        stmt: &Statement
    ) {

        match stmt {

            Statement::VariableDecl {

                name,
                datatype,
                value,
                mutable,

            } => {

                self.generate_variable(
                    name,
                    datatype,
                    value,
                    *mutable,
                );
            }

            Statement::FunctionDecl {

                name,
                parameters,
                return_type,
                body,

            } => {

                self.generate_function(
                    name,
                    parameters,
                    return_type,
                    body,
                );
            }

            Statement::ReturnStmt(expr) => {

                self.output.push_str("return");

                if let Some(e) = expr {

                    self.output.push(' ');

                    self.output.push_str(
                        &self.generate_expression(e)
                    );
                }

                self.output.push_str(";\n");
            }

            Statement::ExpressionStmt(expr) => {

                self.output.push_str(
                    &self.generate_expression(expr)
                );

                self.output.push_str(";\n");
            }

            _ => {}
        }
    }
        fn generate_variable(
        &mut self,
        name: &String,
        datatype: &Option<String>,
        value: &Expression,
        mutable: bool,
    ) {

        self.output.push_str("let ");

        if mutable {

            self.output.push_str("mut ");
        }

        self.output.push_str(name);

        if let Some(dtype) = datatype {

            self.output.push_str(": ");

            let rust_type = match dtype.as_str() {

                "int" => "i64",

                "float" => "f64",

                "bool" => "bool",

                "string" => "String",

                _ => "()",
            };

            self.output.push_str(rust_type);
        }

        self.output.push_str(" = ");

        self.output.push_str(
            &self.generate_expression(value)
        );

        self.output.push_str(";\n");
    }
        fn generate_function(
        &mut self,
        name: &String,
        parameters: &Vec<Parameter>,
        return_type: &Option<String>,
        body: &Vec<Statement>,
    ) {

        self.output.push_str("fn ");

        self.output.push_str(name);

        self.output.push('(');

        for (i, param) in parameters.iter().enumerate() {

            self.output.push_str(
                &param.name
            );

            if let Some(dtype) = &param.datatype {

                self.output.push_str(": ");

                let rust_type = match dtype.as_str() {

                    "int" => "i64",

                    "float" => "f64",

                    "bool" => "bool",

                    "string" => "&str",

                    _ => "()",
                };

                self.output.push_str(rust_type);
            }

            if i < parameters.len() - 1 {

                self.output.push_str(", ");
            }
        }

        self.output.push(')');

        if let Some(ret) = return_type {

            self.output.push_str(" -> ");

            let rust_ret = match ret.as_str() {

                "int" => "i64",

                "float" => "f64",

                "bool" => "bool",

                "string" => "String",

                _ => "()",
            };

            self.output.push_str(rust_ret);
        }

        self.output.push_str(" {\n");

        for stmt in body {

            self.generate_statement(stmt);
        }

        self.output.push_str("}\n");
    }
        fn generate_expression(
        &self,
        expr: &Expression
    ) -> String {

        match expr {

            Expression::Integer(v) => {

                v.to_string()
            }

            Expression::Float(v) => {

                v.to_string()
            }

            Expression::String(v) => {

                format!("String::from(\"{}\")", v)
            }

            Expression::Boolean(v) => {

                v.to_string()
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
                    self.generate_expression(left),
                    operator,
                    self.generate_expression(right)
                )
            }

            Expression::FunctionCall {

                name,
                arguments,

            } => {

                let mut result =
                    format!("{}(", name);

                for (i, arg) in arguments.iter().enumerate() {

                    result.push_str(
                        &self.generate_expression(arg)
                    );

                    if i < arguments.len() - 1 {

                        result.push_str(", ");
                    }
                }

                result.push(')');

                result
            }

            _ => String::from("()"),
        }
    }
}
