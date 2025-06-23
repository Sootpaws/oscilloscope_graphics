{
    description = "Control oscilliscope XY displays using audio signals";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
            cargo_manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in {
            defaultPackage = pkgs.rustPlatform.buildRustPackage {
                pname = cargo_manifest.package.name;
                version = cargo_manifest.package.version;
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
                buildInputs = with pkgs; [
                    alsa-lib
                ];
                nativeBuildInputs = with pkgs; [
                    pkg-config
                    # Development only
                    # TODO: find a better place to put these
                    clippy
                    rustfmt
                ];
            };
        });
}
