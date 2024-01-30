use workspace::database::Database;

pub async fn command(db: &Database, name: &String) -> Result<(), Box<dyn std::error::Error>> {
    if !db.has_with_name(name.clone()).await? {
        return Err("Workspace does not exist".into());
    }

    db.remove(name.clone()).await?;

    println!("Removed {} as a workspace", name);

    Ok(())
}
