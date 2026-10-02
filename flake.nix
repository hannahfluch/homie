{
  description = "Homie, an interactive coding buddy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { fenix, flake-utils, naersk, nixpkgs, ... }:
    # GTK4 layer shell requires Linux and a Wayland session.
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "clippy"
          "rustc"
          "rustfmt"
          "rust-src"
        ];
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
        buildArgs = {
          src = pkgs.lib.cleanSource ./.;
          nativeBuildInputs = with pkgs; [ pkg-config autoPatchelfHook wrapGAppsHook4 ];
          buildInputs = with pkgs; [ gtk4 gtk4-layer-shell ];
        };
        homie = naersk'.buildPackage buildArgs;
      in {
        packages.default = homie;

        apps.default = {
          type = "app";
          program = "${homie}/bin/homie";
        };

        checks.clippy = naersk'.buildPackage (buildArgs // {
          mode = "clippy";
        });

        devShells.default = pkgs.mkShell {
          inputsFrom = [ homie ];
          packages = [ toolchain ];
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      });
}
