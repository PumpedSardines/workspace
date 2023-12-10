use std::ffi::OsString;
use std::path::PathBuf;

use clap::{arg, Command};

mod nix;

fn cli() -> Command {
    Command::new("workspace")
        .about("Manages workspaces")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new("nix")
                .about("Creates required nix files")
                .arg(arg!(-t --template <TEMPLATE> "The template to use"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("nix", sub_matches)) => {
            let template = sub_matches.get_one::<String>("template").unwrap();
            nix::parse_cmd(template);
        }
        Some((cmd, _)) => {
            println!("Unknown subcommand: {}", cmd);
            std::process::exit(1);
        }
        None => {
            println!("No subcommand was used");
            std::process::exit(1);
        }
    }

    // Continued program logic goes here...
}
