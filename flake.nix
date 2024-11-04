{
  description = "something";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        rustToolchain = fenix.packages.${system}.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustc-codegen-cranelift"
          "rustfmt"
        ];

        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustToolchain);

        src = craneLib.cleanCargoSource ./.;

        # Common build environment arguments
        commonArgs = with pkgs; {
          inherit src;
          strictDeps = true;
          buildInputs = [
            trunk
            rustToolchain
            clang
            rust-analyzer-nightly

            # Bevy dependencies
            alsa-lib
            udev
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
          ];

          nativeBuildInputs = [ pkg-config ] ++ lib.catAttrs system [ mold ];

          shellHook = ''
            export LD_LIBRARY_PATH="${
              lib.makeLibraryPath [
                alsa-lib
                udev
                vulkan-loader
                libxkbcommon
                wayland
              ]
            }"
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${clang}/bin/clang"
            export CARGO_ENCODED_RUSTFLAGS="-Clink-arg=-fuse-ld=${mold}/bin/mold"
          '';
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        grave_matters =
          craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in {
        checks = {
          inherit grave_matters;

          # Clippy check
          grave_matters-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Documentation generation
          grave_matters-doc =
            craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });

          # Formatting check
          grave_matters-fmt = craneLib.cargoFmt { inherit src; };

          # TOML formatting check
          grave_matters-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
          };

          my-crate-audit = craneLib.cargoAudit { inherit src advisory-db; };

          # License checks with cargo-deny
          grave_matters-deny = craneLib.cargoDeny { inherit src; };

          # Test execution with cargo-nextest
          grave_matters-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages.default = grave_matters;

        apps.default = flake-utils.lib.mkApp { drv = grave_matters; };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [ ];
        };
      });
}
