{
  description = ''
    Generate daily spotify playlist
  '';


  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    src = {
      url = "path:./";
      flake = false;
    };
  };


  outputs = { nixpkgs, ... }@inputs: let

    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

  in with pkgs; {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {

      pname = "daily-playlist";
      version = "1.0.0";

      src = inputs.src;

      useFetchCargoVendor = true;
      cargoHash = "sha256-3eGj2YedGeYFdN7X6RoSrQj0oh7siFTc6+w7fzENVyM=";
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
