{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    perSystem = { pkgs, system, ... }: rec {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ (import inputs.rust-overlay) ];
      };

      packages.default = pkgs.callPackage ./leptos-test.nix { };
      devShells.default = pkgs.mkShell {
        name = "leptos-test-devShell";
        inputsFrom = [ packages.default ];
        packages = [ pkgs.cargo-leptos ];
      };
    };
  };
}
