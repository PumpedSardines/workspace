mod rust;
mod template;

pub fn parse_cmd(template: &String) {
    match template.as_str() {
        "rust" => {
            rust::create();
        }
        _ => {
            println!("Unknown template: {}", template);
            std::process::exit(1);
        }
    }

    println!("Creating nix files with template: {}", template);
}
