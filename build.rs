use std::{env, fs};

use clap::{CommandFactory, ValueEnum};

include!("src/args.rs");

fn main() {
    // get target directory
    let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target");

    // generate manual
    let manual_dir = target_dir.join("manual");
    fs::create_dir_all(&manual_dir).unwrap();
    println!(
        "man generated at {:?}",
        clap_mangen::generate_to(Args::command(), manual_dir).unwrap()
    );

    // generate completions
    let completion_dir = target_dir.join("completion");
    fs::create_dir_all(&completion_dir).unwrap();
    for &shell in clap_complete::Shell::value_variants() {
        println!(
            "completions generated at {:?}",
            clap_complete::generate_to(shell, &mut Args::command(), "tw", &completion_dir).unwrap()
        );
    }
}
