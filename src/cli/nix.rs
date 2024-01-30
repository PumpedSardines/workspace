pub fn command(template: &String) -> Result<(), Box<dyn std::error::Error>> {
    workspace::nix::parse_cmd(template);

    Ok(())
}
