mod javascript;
mod rust;

mod template;

pub fn parse_cmd(template: &String) {
    let template_name = template.clone();
    let template = match template.as_str() {
        "rust" => rust::template(),
        "javascript" => javascript::template(),
        _ => {
            println!("Unknown template: {}", template);
            std::process::exit(1);
        }
    };

    println!("Creating nix files with template: {}", template_name);
    template.create();
}
