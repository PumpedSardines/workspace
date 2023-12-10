use super::template::Template;

pub fn template() {
    let mut template = Template::new();
    template.add_file(
        "default.nix".to_string(),
        r#"{ pkgs ? import <nixpkgs> { } }:
        pkgs.rustPlatform.buildRustPackage rec {
          pname = \"name\";
          version = \"0.1\";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        }
        "#
        .to_string(),
    );
}
