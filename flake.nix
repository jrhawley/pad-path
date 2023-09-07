{
  description = "pad-path development environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
  };

  outputs = { self, flake-utils, fenix, nixpkgs, }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      toolchain = (with fenix.packages.${system}; combine [
        minimal.rustc
        minimal.cargo
        targets.x86_64-pc-windows-gnu.latest.rust-std
      ]);

      deps = (with pkgs; [
        llvmPackages_16.clangUseLLVM
        libxml2
        openssl
        pkg-config
        cargo-deny
        cargo-edit
        cargo-make
        cargo-nextest
        cargo-watch
        rust-analyzer
        rustup
        xz
      ]);
      dev-deps = with pkgs; [
        cachix
        jq
        p7zip
      ];

      build-deps = with pkgs; [
        pkgsCross.mingwW64.stdenv.cc
        pkgsCross.mingwW64.windows.pthreads
      ];
    in {
      devShells.default = pkgs.mkShell {
        packages = deps ++ dev-deps ++ build-deps;
      };
    });
}
