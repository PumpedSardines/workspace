use clap::{arg, Command};

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
        .subcommand(Command::new("list").about("List all workspaces"))
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
    let matches = cli().get_matches();

    // let db_url = &env::var("DATABASE_URL").expect("There is no DATABASE_URL env variable");
    // let pool = SqlitePool::connect(db_url)
    //     .await
    //     .expect("Couldn't connect to the sqlite db");

    match matches.subcommand() {
        Some(("nix", sub_matches)) => {
            let template = sub_matches.get_one::<String>("template").unwrap();
            workspace::nix::parse_cmd(template);
        }
        Some(("list", _)) => {
            let workspaces = workspace::paths::legacy::load();
            if workspaces.is_empty() {
                println!("No workspaces made");
                std::process::exit(0);
            }
            for workspace in workspaces {
                println!("{}: {}", workspace.name, workspace.path);
            }
        }
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let current_dir = std::env::current_dir().unwrap();
            let workspaces = workspace::paths::legacy::load();
            let mut new_workspaces = workspaces.clone();
            if workspaces.iter().any(|w| &w.name == name) {
                println!("Workspace with name {} already exists", name);
                std::process::exit(1);
            }
            if workspaces
                .iter()
                .any(|w| &w.path == current_dir.to_str().unwrap())
            {
                println!(
                    "Workspace with path {} already exists",
                    current_dir.to_str().unwrap()
                );
                std::process::exit(1);
            }
            new_workspaces.push(workspace::paths::legacy::Workspace {
                name: name.clone(),
                path: current_dir.to_str().unwrap().to_string(),
            });
            
            workspace::paths::legacy::save(new_workspaces);
            println!("Added {} as a workspace", name);
        }
        Some(("rm", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let workspaces = workspace::paths::legacy::load();
            if workspaces.iter().find(|w| &w.name == name).is_none() {
                println!("Workspace with name \"{}\" doesn't exists", name);
                std::process::exit(1);
            }
            workspace::paths::legacy::save(
                workspaces.into_iter().filter(|w| &w.name != name).collect(),
            );
            println!("Remove workspace {}", name);
        }
        Some(("open", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let workspaces = workspace::paths::legacy::load();
            let workspace = workspaces.iter().find(|w| &w.name == name);
            if workspace.is_none() {
                println!("Workspace with name \"{}\" doesn't exists", name);
                std::process::exit(1);
            }
            let workspace = workspace.unwrap();

            let output = std::process::Command::new("tmux")
                .arg("new-session")
                .arg("-s")
                .arg(name)
                .arg("-c")
                .arg(&workspace.path)
                .arg("-d")
                .output()
                .unwrap();

            if !output.status.success() {
                use std::io::Write;
                std::io::stdout().write_all(&output.stdout).unwrap();
                std::io::stderr().write_all(&output.stderr).unwrap();
                std::process::exit(1);
            } else {
                println!("Opened workspace {}", name);
            }
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
