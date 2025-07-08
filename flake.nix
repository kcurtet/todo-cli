{
  description = "A fast, colorful, and feature-rich personal task management CLI tool";

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    utils,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
      in {
        packages.default = naersk-lib.buildPackage {
          src = ./.;
          pname = "todo";
          nativeBuildInputs = with pkgs; [installShellFiles];
          cargoToml = ./Cargo.toml;
          targets = ["todo"];

          meta = with pkgs.lib; {
            description = "A fast, colorful, and feature-rich personal task management CLI tool";
            homepage = "https://github.com/kcurtet/todo-cli";
            license = licenses.mit;
            maintainers = [];
            platforms = platforms.all;
          };

          postInstall = ''
            for shell in bash zsh fish; do
              $out/bin/todo completions $shell > todo.$shell
              installShellCompletion todo.$shell
            done
          '';
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [pre-commit bacon cargo rustc rustfmt rust-analyzer rustPackages.clippy];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            shellHook = ''
              export CARGO_TARGET_DIR=$PWD/target
              export RUST_BACKTRACE=1
              export RUST_LOG=debug
            '';
          };
      }
    );
}
