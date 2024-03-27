{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Setup nixpkgs
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            rust-overlay.overlays.default
            (final: prev: {
              rustToolchains = {
                stable = prev.rust-bin.stable.latest.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                };
                nightly = prev.rust-bin.nightly.latest.default;
              };
            })
          ];
        };
        # Rust fmt nightly
        rustfmt-nightly = with pkgs;
          writeShellScriptBin "rustfmt-nightly"
            ''exec ${rustToolchains.nightly}/bin/rustfmt'';

        # Setup runtime dependencies
        runtimeInputs = with pkgs; [
          rustToolchains.stable
          openssl.dev
          pkg-config
        ]
        # Some additional libraries for the Darwin platform
        ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];
        # Setup dprint deps
        dprintDeps = with pkgs; [
          dprint
          rustfmt-nightly
          nixpkgs-fmt
        ];

        # CI scripts
        ci = with pkgs; {
          fmt = writeShellApplication {
            name = "ci-fmt";
            runtimeInputs = dprintDeps;
            text = ''dprint fmt'';
          };

          check_fmt = writeShellApplication {
            name = "ci-check-fmt";
            runtimeInputs = dprintDeps;
            text = ''dprint check'';
          };

          check_semver = writeShellApplication {
            name = "ci-check-semver";
            runtimeInputs = with pkgs; [ cargo-semver-checks ];
            text = ''cargo semver-checks'';
          };

          tests = writeShellApplication {
            name = "ci-run-tests";
            inherit runtimeInputs;
            text = ''
              cargo test --workspace --all-features --all-targets
            '';
          };

          lints = writeShellApplication {
            name = "ci-run-lints";
            inherit runtimeInputs;
            text = ''
              cargo clippy --workspace --all-features --all --all-targets
              cargo doc --workspace --all-features  --no-deps
            '';
          };
          # Run them all together
          all = writeShellApplication {
            name = "ci-run-all";
            runtimeInputs = [ ci.lints ci.tests ci.check_fmt ci.check_semver ];
            text = ''
              ci-check-fmt
              ci-run-tests
              ci-run-lints
              ci-check-semver
            '';
          };
        };
      in
      {
        # for `nix fmt`
        formatter = ci.fmt;
        # for `nix flake check`
        # checks = {
        #   formatting = ci.fmt_check;
        #   semver = ci.semver_check;
        # };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = runtimeInputs ++ [
            dprintDeps
            ci.all
            ci.lints
            ci.tests
          ];
        };

        # Nightly compilator to run miri tests
        devShells.nightly = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchains.nightly
          ];
        };

        packages = {
          ci-lints = ci.lints;
          ci-tests = ci.tests;
          ci-check-fmt = ci.check_fmt;
          ci-check-semver = ci.check_semver;
          ci-all = ci.all;
        };
      });
}
