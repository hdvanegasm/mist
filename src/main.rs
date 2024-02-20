use std::fmt::Display;

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;

/// Simple CLI to encrypt files using a key file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    function: Function,
}

/// Functionalities available in the tool.
#[derive(Debug, Clone, Subcommand)]
enum Function {
    /// Generates a random key and stores it in the given file path.
    Gen {
        /// Path to the file to store the key.
        #[arg(short, long)]
        file: String,
    },

    /// To encrypt the file.
    Enc {
        /// Path to the file containing the secret key.
        #[arg(short, long)]
        key: String,

        /// Path to the file to be encrypted.
        #[arg(short, long)]
        file: String,
    },

    /// To decrypt the file.
    Dec {
        /// Path to the file containing the secret key.
        #[arg(short, long)]
        key: String,

        /// Path to the file to be encrypted.
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.function {
        Function::Gen { file } => todo!(),
        Function::Enc { key, file } => todo!(),
        Function::Dec { key, file } => todo!(),
    }

    env_logger::init_from_env(Env::default().filter_or("LOG_lEVEL", "info"));
}
