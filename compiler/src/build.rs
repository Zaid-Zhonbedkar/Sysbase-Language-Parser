use std::process::Command;

pub fn compile_generated_rust() {

    let status = Command::new("rustc")

        .arg("build/main.rs")

        .arg("-o")

        .arg("build/nexis_app")

        .status()

        .expect(
            "Failed to execute rustc"
        );

    if status.success() {

        println!(
            "Rust compilation successful."
        );

    } else {

        println!(
            "Rust compilation failed."
        );
    }
}