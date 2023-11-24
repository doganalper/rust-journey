// this function is the first function to run when compiled program runs
fn main() {
    // this is a macro not a function
    // ! sign differenciates between macro and function
    println!("Hello, world!");
}

// for program to work it needs to be compiled first by "rustc <main-file-path>"

// CARGO:
// Cargo is Rust's build system and package manager.
// Some of it's abilities are but not limited to: build code, download libraries and building these libraries
// To create new Rust project "cargo new <project-name>" is the command you need.
// This command also creates a git repository on local
// Cargo also creates "Cargo.toml" file that has contents about this project.

// BUILDING:
// Run "cargo build" to build your project. This command creates a file in target/debug/hello_cargo.
// This builds is a debug build.
// Addition to building and running output file seperately "cargo run" does these two steps in order.

// COMPILE CHECK:
// Also there is "cargo check" command that checks if project can compile or not but does not product an executable.
