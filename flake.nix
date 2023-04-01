{
  description = "pad-path development environment";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    # rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      # overlays = [
      #   (import rust-overlay)
      #   (self: super: {
      #     rustToolchain =
      #       let
      #         rust = super.rust-bin;
      #       in
      #       if builtins.pathExists ./rust-toolchain.toml then
      #         rust.fromRustupToolchainFile ./rust-toolchain.toml
      #       else if builtins.pathExists ./rust-toolchain then
      #         rust.fromRustupToolchainFile ./rust-toolchain
      #       else
      #         rust.stable.latest.default;
      #   })
      # ];

      pkgs = import nixpkgs { inherit system; }; #overlays; };
      rustToolchain = fenix.packages.${system}.stable.toolchain;
      rustPlatform = (pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      });

      name = "pad-path";
      version = "0.2.5";
      deps = with pkgs; [
        rustToolchain
        openssl
        pkg-config
        cargo-deny
        cargo-edit
        cargo-make
        cargo-nextest
        cargo-watch
        rust-analyzer
      ];    
      
      drv = rustPlatform.buildRustPackage {
        pname = "${name}";
        version = "${version}";
        src = builtins.path {
          path = ./.;
          name = "${name}";
        };
        cargoLock.lockFile = ./Cargo.lock;
      };
    in {
      packages = {
        "${name}" = drv;
        default = drv;
      };
      devShells.default = pkgs.mkShell {
        packages = deps;
      };
    });
}
