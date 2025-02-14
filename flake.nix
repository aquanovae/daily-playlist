{
  description = ''
    Generate daily spotify playlist
  '';


  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";


  outputs = { nixpkgs, ... }: let

    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

  in with pkgs; {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {

      pname = "daily-playlist";
      version = "1.0.0";

      src = "./";

      useFetchCargoVendor = true;
      cargoHash = "";
    };


    devShells.${system}.default = pkgs.stdenv.mkDerivation {

      name = "rust";

      nativeBuildInputs = [
        pkg-config
      ];

      buildInputs = [ 
        cargo
        rustc

        openssl
      ];
    };
  };
}
