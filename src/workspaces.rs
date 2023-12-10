use homedir::get_my_home;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: String,
}

pub fn load() -> Vec<Workspace> {
    let current_dir = get_my_home().unwrap().unwrap();
    let file = current_dir.join(".workspaces.json");

    let mut workspaces = if file.exists() {
        let data = fs::read_to_string(file).unwrap();
        serde_json::from_str::<Vec<Workspace>>(&data).unwrap_or(vec![])
    } else {
        fs::write(file, "[]").unwrap();
        vec![]
    };

    workspaces.sort_by(|a, b| a.name.cmp(&b.name));

    workspaces
}
pub fn save(workspaces: Vec<Workspace>) {
    let current_dir = get_my_home().unwrap().unwrap();
    let file = current_dir.join(".workspaces.json");

    let data = serde_json::to_string(&workspaces).unwrap();
    fs::write(file, data).unwrap();
}
