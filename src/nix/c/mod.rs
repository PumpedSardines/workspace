use super::template::Template;

const FLAKE: &str = include_str!("./flake.nix");
const DEFAULT: &str = include_str!("./default.nix");

pub fn template() -> Template {
    let mut template = Template::new();

    template.add_file("default.nix".to_string(), DEFAULT.to_string());
    template.add_file("flake.nix".to_string(), FLAKE.to_string());

    template
}
