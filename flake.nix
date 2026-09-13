{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              lld
              nodejs
              pkg-config
              rustc
              rustfmt
              rust-analyzer
              wasm-pack

              # Debug/profiling tools
              hotspot
              perf
            ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        };
    };
}
