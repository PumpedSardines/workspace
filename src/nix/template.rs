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
        for file in &self.files {
            println!("Creating file: {}", file.name);
            println!("With content: {}", file.content);
        }
    }
}
