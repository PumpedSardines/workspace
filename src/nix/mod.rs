mod c;
mod javascript;
mod rust;

mod template;

pub const TEMPLATES: [&'static str; 3] = ["rust", "javascript", "c"];

pub fn parse_cmd(template: &String) {
    let template_name = template.clone();
    let template = match template.as_str() {
        "rust" => rust::template(),
        "javascript" => javascript::template(),
        "c" => c::template(),
        _ => {
            println!("Unknown template: {}", template);
            std::process::exit(1);
        }
    };

    println!("Creating nix files with template: {}", template_name);
    template.create();
}
