use clap::CommandFactory;
use clap_mangen::Man;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use tabiew::args::Args;

/// Environment variable for the output directory.
const OUT_DIR_ENV: &str = "OUT_DIR";

/// Man page can be created with:
///
/// ```sh
/// cargo run --bin tabiew-mangen -F clap_mangen
/// ````
///
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var(OUT_DIR_ENV).unwrap_or_else(|_| panic!("{OUT_DIR_ENV} is not set"));
    let out_path = PathBuf::from(out_dir).join(format!("{}.1", env!("CARGO_PKG_NAME")));
    let app = Args::command();
    let man = Man::new(app);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(&out_path, buffer)?;
    println!("Man page is generated at {out_path:?}");
    Ok(())
}
