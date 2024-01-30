use tmux_interface::{AttachSession, HasSession, NewSession, SwitchClient, Tmux};
use workspace::database::Database;

pub async fn command(db: &Database, name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let workspace = db.get(name.clone()).await?;

    if workspace.is_none() {
        return Err(format!("Workspace with name \"{}\" doesn't exists", name).into());
    }
    let workspace = workspace.unwrap();

    let session_name = name;

    let session_opened = Tmux::with_command(HasSession::new().target_session(session_name))
        .status()?
        .success();

    if session_opened {
        return Err(format!("Workspace with name \"{}\" is already opened", name).into());
    }

    Tmux::new()
        .add_command(
            NewSession::new()
                .detached()
                .session_name(session_name)
                .start_directory(&workspace.path),
        )
        .output()?;

    Tmux::new()
        .add_command(SwitchClient::new().target_session(session_name))
        .add_command(AttachSession::new().target_session(session_name))
        .output()?;

    println!("Opened workspace {}", name);

    Ok(())
}
