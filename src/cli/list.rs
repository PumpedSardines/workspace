use workspace::database::Database;

pub async fn command(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let workspaces = db.list().await.unwrap();

    if workspaces.is_empty() {
        return Err("No workspaces made".into());
    }

    for workspace in workspaces {
        println!("{}: {}", workspace.name, workspace.path);
    }

    Ok(())
}
