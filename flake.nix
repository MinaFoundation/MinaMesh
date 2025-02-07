{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;

        unfilteredRoot = ./.;
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            # Also keep any graphql files
            (lib.fileset.fileFilter (file: file.hasExt "graphql")
              unfilteredRoot)
            # SQL files
            ./sql
            # Static resources
            (lib.fileset.maybeMissing ./static)
          ];
        };

        # Common arguments
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs;
            [
              # Additional build inputs
              openssl
            ] ++ lib.optionals pkgs.stdenv.isDarwin
            (with pkgs.darwin.apple_sdk.frameworks; [ IOKit Security ]);

          # Additional runtime dependencies
          nativeBuildInputs = with pkgs; [ pkg-config sqlx-cli ];
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        mina-mesh =
          craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit mina-mesh;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          mina-mesh-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          mina-mesh-doc =
            craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });

          # Check formatting
          mina-mesh-fmt = craneLib.cargoFmt { inherit src; };

          mina-mesh-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          mina-mesh-audit = craneLib.cargoAudit { inherit src advisory-db; };

          # Audit licenses
          mina-mesh-deny = craneLib.cargoDeny { inherit src; };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          mina-mesh-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
          });
        };

        packages = {
          default = mina-mesh;
        } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          mina-mesh-llvm-coverage = craneLibLLvmTools.cargoLlvmCov
            (commonArgs // { inherit cargoArtifacts; });
        };

        apps.default = flake-utils.lib.mkApp { drv = mina-mesh; };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = with pkgs; [
            just
            nixd # Nix language server
            nixfmt # Nix formatter
            cargo-watch
            postgresql
          ];
        };
      });
}
