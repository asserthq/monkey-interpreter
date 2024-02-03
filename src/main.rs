pub mod token;
pub mod lexer;
pub mod repl;

fn main() -> std::io::Result<()> {
    println!("Hello, monkey!");
    repl::start()?;
    Ok(())
}
