{
  description = "Generate daily spotify playlist";


  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";


  outputs = { nixpkgs, ... }: let

    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    buildInputs = with pkgs; [
      openssl
    ];
    nativeBuildInputs = with pkgs; [
      cargo
      cargo-edit
      rustc
      pkg-config
    ];

  in {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "daily-playlist";
      version = "1.1.4";
      src = ./.;

      inherit buildInputs nativeBuildInputs;

      useFetchCargoVendor = true;
      cargoHash = "sha256-GlQm6c1cpL3BEVONOYOkL80KfaXVJe3dZRYjyNVrEdI=";
    };


    devShells.${system}.default = pkgs.stdenv.mkDerivation {
      name = "rust";

      inherit buildInputs nativeBuildInputs;
    };
  };
}
