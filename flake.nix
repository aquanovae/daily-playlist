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
      rustc
      pkg-config
    ];

  in {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "daily-playlist";
      version = "1.1.3";
      src = ./.;

      inherit buildInputs nativeBuildInputs;

      useFetchCargoVendor = true;
      cargoHash = "sha256-peMtNyAlVpXnh6so7aHHshSJb92hoEb4gGHNFHNSyBU=";
    };


    devShells.${system}.default = pkgs.stdenv.mkDerivation {
      name = "rust";

      inherit buildInputs nativeBuildInputs;
    };
  };
}
