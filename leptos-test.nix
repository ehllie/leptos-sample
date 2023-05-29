{ makeRustPlatform
, rust-bin
, sass
,
}:
let
  rust = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  inherit (makeRustPlatform {
    cargo = rust;
    rustc = rust;
  }) buildRustPackage;
in
buildRustPackage {
  pname = "leptos-test";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  nativeBuildInputs = [ sass ];
}
