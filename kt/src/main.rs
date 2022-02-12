use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// File to print
    #[clap(parse(from_os_str))]
    file: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Some(file) = cli.file.as_deref() {
        if file.exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("[Error] Unable to read the  file.");
                    let stdout = std::io::stdout(); // get the global stdout entity
                    let mut handle = std::io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {}
                        Err(err) => {
                            eprintln!("[Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("[Error] No such file or directory.");
            process::exit(1);
        }
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}
