{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-root.url = "github:srid/flake-root";
  };

  outputs = inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.flake-root.flakeModule
      ];

      systems = nixpkgs.lib.systems.flakeExposed;

      flake = { };

      perSystem = { config, self', inputs', system, nixpkgs, pkgs, ... }: {
        # Setup nixpkgs with overlays.
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
            (final: prev: {
              rustToolchains = {
                stable = pkgs.rust-bin.stable.latest.default;
                nightly = pkgs.rust-bin.nightly.latest.default;
              };
            })
          ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; let
            # Scripts used in CI
            ci-run-tests = writeShellApplication {
              name = "ci-run-tests";
              runtimeInputs = [
                rustToolchains.stable
              ];
              text = ''
                cargo test --workspace --all-features --all-targets
                # TODO Add cargo publish test with the cargo workspaces
              '';
            };

            ci-run-lints = writeShellApplication {
              name = "ci-run-lints";
              runtimeInputs = [
                rustToolchains.stable
              ];
              text = ''
                cargo clippy --workspace --all-features --all --all-targets
                cargo doc --workspace --all-features  --no-deps
              '';
            };

            # Run them all together
            ci-run-all = writeShellApplication {
              name = "ci-run-all";
              runtimeInputs = [
                ci-run-tests
                ci-run-lints
              ];
              text = ''
                ci-run-tests
                ci-run-lints
              '';
            };
          in
          [
            rustToolchains.stable
            cargo-workspaces

            ci-run-tests
            ci-run-lints
            ci-run-all
          ];
        };

        # Nightly compilator to run miri tests
        devShells.nightly = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchains.nightly
          ];
        };

        treefmt.config = {
          inherit (config.flake-root) projectRootFile;

          programs.nixpkgs-fmt.enable = true;
          programs.rustfmt = {
            enable = true;
            package = pkgs.rustToolchains.nightly;
          };
          programs.beautysh.enable = true;
          programs.deno.enable = true;
          programs.taplo.enable = true;
        };

        formatter = config.treefmt.build.wrapper;
      };
    };
}
