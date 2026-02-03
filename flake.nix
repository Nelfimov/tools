{
  description = "Tools";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };
  outputs =
    {
      flake-utils,
      nixpkgs,
      fenix,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        fenixLib = fenix.packages.${system};
        rustToolchain = fenixLib.latest.withComponents [
          "cargo"
          "rustc"
          "clippy"
          "rustfmt"
          "rust-analyzer"
          "rust-src"
        ];
        naerskLib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        readCargoToml =
          path:
          let
            toml = builtins.fromTOML (builtins.readFile path);
          in
          {
            version = toml.package.version or "unknown";
            inherit (toml.package) name;
          };
        cargo = readCargoToml ./entrypoint/Cargo.toml;
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rustToolchain
          ];

          shellHook = ''
            unset DEVELOPER_DIR;
            if test -f ".env"; then
              set -a
              source .env
              set +a
            fi
          '';
        };

        defaultPackage = naerskLib.buildPackage {
          inherit (cargo) name;
          inherit (cargo) version;
          pname = cargo.name;
          src = ./.;
          cargoBuildOptions =
            opts:
            opts
            ++ [
              "-p"
              cargo.name
            ];
        };
      }
    );
}
