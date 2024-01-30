use super::utils::validate_name;
use workspace::database::Database;

pub async fn command(db: &Database, name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir().unwrap();

    if !validate_name(name) {
        return Err(format!("Invalid workspace name: \"{}\"", name).into());
    }

    if db.has_with_name(name.clone()).await? {
        return Err("Workspace already exists".into());
    }

    if db
        .has_with_path(current_dir.to_str().unwrap().to_string())
        .await?
    {
        return Err("Workspace with path already exists".into());
    }

    db.add(name.clone(), current_dir.to_str().unwrap().to_string())
        .await?;

    println!("Added {} as a workspace", name);

    Ok(())
}
