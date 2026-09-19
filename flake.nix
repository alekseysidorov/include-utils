{
  description = "A powerful, section-aware replacement for Rust's include_str macro.";

  inputs = {
    # Keep the compiler and Nix package set on the same stable channel.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-parts.url = "github:hercules-ci/flake-parts";

    # Reuse the shared Rust overlay and project-source helper.
    nix-devtools = {
      url = "github:alekseysidorov/nix-devtools";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-parts.follows = "flake-parts";
      inputs.treefmt-nix.follows = "treefmt-nix";
    };

    crane.url = "github:ipetkov/crane";
    rust-advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
        # Keep provider-owned inputs available to the composed flake modules.
        specialArgs.localInputs = inputs;
      }
      (
        { ... }:
        let
          inherit (inputs.nixpkgs) lib;
          # Use nix-devtools' public overlay so rust-bin has one shared owner.
          defaultOverlay = inputs.nix-devtools.overlays.default;
        in
        {
          systems = lib.systems.flakeExposed;
          imports = [
            inputs.treefmt-nix.flakeModule
            inputs.nix-devtools.flakeModule
          ];
          flake.overlays.default = defaultOverlay;

          perSystem =
            { system, ... }:
            let
              # Build checks and shells from one consistently extended package set.
              pkgs = inputs.nixpkgs.legacyPackages.${system}.extend inputs.self.overlays.default;
              rustToolchain = pkgs.rust-bin.stable.latest.default.override {
                extensions = [
                  "clippy"
                  "rust-src"
                  "rustfmt"
                ];
              };
              craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
              # Respect .gitignore while retaining all project files needed by Cargo.
              src = pkgs.projectSource { projectRoot = ./.; };
              commonArgs = {
                inherit src;
                pname = "include-utils";
                version = "0.2.6";
                strictDeps = true;
              };
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
              package = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
            in
            {
              treefmt = {
                projectRootFile = "flake.nix";
                programs = {
                  nixfmt.enable = true;
                  rustfmt = {
                    enable = true;
                    package = rustToolchain;
                  };
                  taplo.enable = true;
                };
              };
              packages.default = package;
              checks = {
                build = package;
                test = craneLib.cargoTest (
                  commonArgs
                  // {
                    inherit cargoArtifacts;
                    cargoTestExtraArgs = "--workspace --all-targets --all-features";
                  }
                );
                clippy = craneLib.cargoClippy (
                  commonArgs
                  // {
                    inherit cargoArtifacts;
                    cargoClippyExtraArgs = "--workspace --all-targets --all-features -- -D warnings";
                  }
                );
                doc = craneLib.cargoDoc (
                  commonArgs
                  // {
                    inherit cargoArtifacts;
                    cargoDocExtraArgs = "--workspace --all-features --no-deps";
                  }
                );
                audit = craneLib.cargoAudit {
                  inherit src;
                  advisory-db = inputs.rust-advisory-db;
                };
              };
              devShells.default = pkgs.mkShell {
                packages = [
                  rustToolchain
                  pkgs.cargo-audit
                  pkgs.cargo-nextest
                  pkgs.rust-analyzer
                ];
              };
            };
        }
      );
}
