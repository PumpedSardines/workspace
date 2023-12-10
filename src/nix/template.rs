use std::fs;

pub struct Template {
    files: Vec<File>,
}

pub struct File {
    name: String,
    content: String,
}

impl Template {
    pub fn new() -> Template {
        Template { files: Vec::new() }
    }

    pub fn add_file(&mut self, name: String, content: String) {
        self.files.push(File { name, content });
    }

    pub fn create(&self) {
        let current_dir = std::env::current_dir().unwrap();
        let mut did_file_exist = false;

        for file in &self.files {
            let file_path = current_dir.join(&file.name);

            if let Ok(_) = fs::metadata(file_path) {
                println!("File already exists: {}", file.name);
                did_file_exist = true;
            }
        }

        if did_file_exist {
            println!("Aborting...");
            std::process::exit(1);
        }

        for file in &self.files {
            let file_path = current_dir.join(&file.name);
            if let Err(e) = fs::write(file_path, &file.content) {
                println!("Error writing file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
