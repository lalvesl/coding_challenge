{
  description = "Prost dependencies";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        rustVersion = cargoToml.package.rust-version;
        default_pkgs = with pkgs; [
          cmake
          pkg-config
          libc
        ];
        base_rust_pkgs = pkgs.rust-bin.stable."${rustVersion}".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
        base_pkgs = default_pkgs ++ [ base_rust_pkgs ];

        mutants_script = pkgs.writeShellScriptBin "mutants" ''
          ${pkgs.cargo-mutants}/bin/cargo-mutants mutants "$@"
        '';

        kani_script = pkgs.writeShellScriptBin "kani" ''
          if ! command -v cargo-kani &> /dev/null; then
            echo "cargo-kani not found. Installing kani-verifier..."
            cargo install kani-verifier --locked
          fi
          # Ensure ~/.cargo/bin is in PATH for this session if not already
          export PATH="$HOME/.cargo/bin:$PATH"
          if ! command -v cargo-kani &> /dev/null; then
             echo "Error: cargo-kani still not found after installation."
             exit 1
          fi
          cargo kani "$@"
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          packages = base_pkgs ++ [
            pkgs.cbmc
            pkgs.cbmc-viewer
          ];
        };
        devShells."stable" =
          let
            rustpkgs = pkgs.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            };
          in
          pkgs.mkShell {
            packages = [
              rustpkgs
              pkgs.cbmc
              pkgs.cbmc-viewer
            ]
            ++ default_pkgs;
          };
        packages.default =
          let
            rustpkgs = base_rust_pkgs;
            rustPlatform = pkgs.makeRustPlatform {
              cargo = rustpkgs;
              rustc = rustpkgs;
            };
          in
          rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.cmake
            ];
            dontUseCmakeConfigure = true;
            buildInputs = [
              pkgs.protobuf
              pkgs.curl
              pkgs.libc
            ];
          };

        apps.mutants = {
          type = "app";
          program = "${mutants_script}/bin/mutants";
        };

        apps.kani = {
          type = "app";
          program = "${kani_script}/bin/kani";
        };
      }
    );
}
