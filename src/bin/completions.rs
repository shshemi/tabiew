use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use std::env;
use std::io::Result;
use tabiew::args::Args;

/// Environment variable for the output directory.
const OUT_DIR_ENV: &str = "OUT_DIR";

/// Shell completions can be created with:
///
/// ```sh
/// cargo run --bin tabiew-completions -F clap_complete
/// ```
///
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var(OUT_DIR_ENV).unwrap_or_else(|_| panic!("{OUT_DIR_ENV} is not set"));
    let mut app = Args::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts are generated in {out_dir:?}");
    Ok(())
}
