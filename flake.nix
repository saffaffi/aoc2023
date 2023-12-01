{
  description = "aoc2023";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    cargo2nix.inputs.flake-utils.follows = "flake-utils";
    cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , ...
    } @ inputs:
    let
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [
          inputs.cargo2nix.overlays.default
          inputs.fenix.overlays.default

          (final: prev: {
            rust-toolchain =
              let
                inherit (final.lib) fakeSha256;
                inherit (final.lib.strings) fileContents;

                toolchainFor = target: target.fromToolchainFile {
                  file = ./rust-toolchain.toml;
                  # Replace `fakeSha256` with the hash string produced by Nix
                  # when it tries to build this for the first time.
                  sha256 = "sha256-U2yfueFohJHjif7anmJB5vZbpP7G6bICH4ZsjtufRoU=";
                };

                rustfmt = final.fenix.latest.rustfmt;
              in
              final.fenix.combine [
                rustfmt
                (toolchainFor final.fenix)
              ];
          })

          (final: prev: {
            cargo2nix = inputs.cargo2nix.packages.${system}.default;
          })
        ];
      };

      supportedSystems = with flake-utils.lib.system; [
        aarch64-darwin
        x86_64-darwin
        x86_64-linux
      ];

      inherit (flake-utils.lib) eachSystem;
    in
    eachSystem supportedSystems (system:
    let
      pkgs = pkgsFor system;

      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustToolchain = pkgs.rust-toolchain;
      };
    in
    rec
    {
      packages = rec {
        default = aoc2023;
        aoc2023 = (rustPkgs.workspace.aoc2023 { }).out;
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo2nix
          rust-toolchain

          aoc-cli

          libiconv
        ];
      };

      formatter = pkgs.nixpkgs-fmt;
    });
}
