mod parser;
mod ast;
mod codegen;

use std::env;
use std::fs;

use std::process::Command;

use pest::Parser;

use parser::{
    NexisParser,
    Rule,
};

use parser::build_ast;

use codegen::generate_rust;

fn main() {

    // ==========================================
    // CLI ARGUMENT
    // ==========================================

    let args: Vec<String> =
        env::args().collect();

    if args.len() < 2 {

        println!(
            "Usage: cargo run <file.nx>"
        );

        return;
    }

    let filepath = &args[1];

    // ==========================================
    // READ NEXIS FILE
    // ==========================================

    let source =
        fs::read_to_string(filepath)

        .expect(
            "Failed to read file"
        );

    // ==========================================
    // PARSE
    // ==========================================

    let parsed =
        NexisParser::parse(
            Rule::program,
            &source
        )

        .expect(
            "Parse failed"
        );

    let pair =
        parsed.into_iter()
            .next()
            .unwrap();

    // ==========================================
    // BUILD AST
    // ==========================================

    let ast =
        build_ast(pair);

    println!(
        "\nAST:\n{:#?}",
        ast
    );

    // ==========================================
    // GENERATE RUST
    // ==========================================

    let rust_code =
        generate_rust(&ast);

    println!(
        "\nGenerated Rust:\n{}",
        rust_code
    );

    // ==========================================
    // CREATE BUILD DIR
    // ==========================================

    fs::create_dir_all(
        "build"
    )

    .unwrap();

    // ==========================================
    // WRITE RUST FILE
    // ==========================================

    fs::write(
        "build/main.rs",

        &rust_code
    )

    .unwrap();

    // ==========================================
    // COMPILE GENERATED RUST
    // ==========================================

    let compile =
        Command::new("rustc")

        .arg("build/main.rs")

        .arg("-o")

        .arg("build/app")

        .status();

    match compile {

        Ok(status) => {

            if status.success() {

                println!(
                    "\nCompilation successful."
                );

            } else {

                println!(
                    "\nRust compilation failed."
                );
            }
        }

        Err(_) => {

            println!(
                "\nrustc not installed."
            );
        }
    }

    // ==========================================
    // RUN EXECUTABLE
    // ==========================================

    let run =
        Command::new("build/app")

        .status();

    match run {

        Ok(_) => {

            println!(
                "\nProgram executed."
            );
        }

        Err(_) => {

            println!(
                "\nFailed to run executable."
            );
        }
    }
}