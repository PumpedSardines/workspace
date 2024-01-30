use clap::{arg, Command};

mod cli;

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
        .subcommand(Command::new("ls").about("List all workspaces"))
        .subcommand(
            Command::new("add")
                .about("Add current directory as a workspace")
                .arg(arg!(-n --name <NAME> "The name of the workspace"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a workspace")
                .arg(arg!(-n --name <NAME> "Remove a workspace by name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("open")
                .about("Opens a workspace")
                .arg(arg!(-n --name <NAME> "Opens a workspace by name"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() {
    let db = workspace::database::Database::new()
        .await
        .unwrap_or_else(|e| {
            println!("Couldn't connect to database: {}", e);
            std::process::exit(1);
        });

    db.migrate_legacy_data().await.unwrap_or_else(|e| {
        println!("Couldn't migrate legacy data: {}", e);
        std::process::exit(1);
    });

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("nix", sub_matches)) => {
            let template = sub_matches
                .get_one::<String>("template")
                .expect("Template exists due to clap");

            cli::nix::command(&template).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
        Some(("ls", _)) => {
            cli::list::command(&db).await.unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();

            cli::add::command(&db, &name).await.unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
        Some(("rm", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();

            cli::rm::command(&db, &name).await.unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
        Some(("open", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();

            cli::open::command(&db, &name).await.unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
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
