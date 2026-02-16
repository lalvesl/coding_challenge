{
  description = " dependencies";

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
          coreutils
          prettier
          cargo-mutants
          gnuplot
        ];
        base_rust_pkgs = pkgs.rust-bin.stable."${rustVersion}".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
        base_pkgs = default_pkgs ++ [ base_rust_pkgs ];

      in
      {
        devShells.default = pkgs.mkShell {
          packages = base_pkgs;
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
              pkgs.curl
              pkgs.libc
            ];
          };

        packages.windows =
          let
            crossPkgs = pkgs.pkgsCross.mingwW64;
            target = "x86_64-pc-windows-gnu";
            toolchain = pkgs.rust-bin.stable.${rustVersion}.default.override {
              targets = [ target ];
            };
            myRustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          in
          myRustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-windows";
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [
              crossPkgs.buildPackages.binutils
              crossPkgs.stdenv.cc
            ];
            buildInputs = [
              crossPkgs.windows.pthreads
            ];
            target = target;
            CARGO_BUILD_TARGET = target;
            cargoBuildFlags = [
              "--target"
              target
            ];
            installPhase = ''
              mkdir -p $out/bin
              cp target/${target}/release/${cargoToml.package.name}.exe $out/bin/
            '';
            stdenv = crossPkgs.stdenv;
            dontUseCmakeConfigure = true;
            doCheck = false;
          };

        packages.macos-intel =
          let
            crossPkgs = pkgs.pkgsCross.x86_64-darwin;
            target = "x86_64-apple-darwin";
            toolchain = pkgs.rust-bin.stable.${rustVersion}.default.override {
              targets = [ target ];
            };
            myRustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          in
          myRustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-macos-intel";
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            target = target;
            stdenv = crossPkgs.stdenv;
            dontUseCmakeConfigure = true;
          };

        packages.macos-arm =
          let
            crossPkgs = pkgs.pkgsCross.aarch64-darwin;
            target = "aarch64-apple-darwin";
            toolchain = pkgs.rust-bin.stable.${rustVersion}.default.override {
              targets = [ target ];
            };
            myRustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          in
          myRustPlatform.buildRustPackage {
            pname = "${cargoToml.package.name}-macos-arm";
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            target = target;
            stdenv = crossPkgs.stdenv;
            dontUseCmakeConfigure = true;
          };

        apps.mutants =
          let
            mutants_script = pkgs.writeShellScriptBin "mutants" ''
              ${pkgs.cargo-mutants}/bin/cargo-mutants mutants --in-place "$@"
              rm test_run_inner_*
            '';
          in
          {
            type = "app";
            program = "${mutants_script}/bin/mutants";
          };

        apps."prepare-tests" =
          let
            prepare_tests_script = pkgs.writeShellScriptBin "prepare_tests" ''
              export PATH="${pkgs.lib.makeBinPath base_pkgs}:$PATH"
              cargo build -p test-data-gen
              ./target/debug/test-data-gen "$@"
            '';
          in
          {
            type = "app";
            program = "${prepare_tests_script}/bin/prepare_tests";
          };
      }
    );
}
